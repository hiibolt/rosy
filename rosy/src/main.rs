mod update_check;

use rosy::{ast::{self, FromRule}, embedded, program::Program, resolve, syntax_config, transpile::*};
use anyhow::{Context, Result, anyhow, ensure};
use clap::{Parser as ClapParser, Subcommand};
use pest::Parser;
use std::{fs, fs::write, path::PathBuf, process::Command, time::Instant};
use tracing::info;
use tracing_subscriber;

// ANSI color helpers (stderr only)
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn step(num: usize, total: usize, label: &str) {
    eprint!("{BOLD}{CYAN}[{num}/{total}]{RESET} {label}...");
}
fn step_done(start: Instant) {
    let ms = start.elapsed().as_millis();
    eprintln!(" {GREEN}done{RESET} {DIM}({ms}ms){RESET}");
}
fn step_fail() {
    eprintln!(" {RED}failed{RESET}");
}

/// Rosy Transpiler - Converts Rosy source code to executable Rust programs
#[derive(ClapParser)]
#[command(name = "rosy")]
#[command(version)]
#[command(about = "Rosy Transpiler for beam physics calculations", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Rosy script directly without copying binary to PWD
    Run {
        /// Path to the Rosy source file
        source: PathBuf,

        /// Output directory for build artifacts (default: .rosy_output)
        #[arg(short = 'd', long)]
        output_dir: Option<PathBuf>,

        /// Build in release mode with optimizations
        #[arg(short, long)]
        release: bool,

        /// Aggressive optimizations: LTO, single codegen unit, panic=abort, SIMD DA (slower builds, faster binaries; requires nightly Rust)
        #[arg(long)]
        optimized: bool,

        /// Enforce COSY INFINITY syntax: memory sizes are required in VARIABLE declarations
        #[arg(long)]
        cosy_syntax: bool,
    },

    /// Run language feature tests (transpile, compile, execute each construct)
    Test {
        /// Only run tests whose name contains this string
        #[arg(short, long)]
        filter: Option<String>,

        /// Run tests in release mode
        #[arg(short, long)]
        release: bool,

        /// Number of parallel test workers (each gets its own build directory)
        #[arg(short, long, default_value = "1")]
        parallel: usize,
    },

    /// Build a Rosy script and place the binary in PWD
    Build {
        /// Path to the Rosy source file
        source: PathBuf,

        /// Output binary name (default: source filename without extension)
        #[arg(short, long)]
        output: Option<String>,

        /// Output directory for build artifacts (default: .rosy_output)
        #[arg(short = 'd', long)]
        output_dir: Option<PathBuf>,

        /// Build in release mode with optimizations
        #[arg(short, long)]
        release: bool,

        /// Aggressive optimizations: LTO, single codegen unit, panic=abort, SIMD DA (slower builds, faster binaries; requires nightly Rust)
        #[arg(long)]
        optimized: bool,

        /// Enforce COSY INFINITY syntax: memory sizes are required in VARIABLE declarations
        #[arg(long)]
        cosy_syntax: bool,
    },
}

fn rosy(
    script_path: &PathBuf,
    output_dir: Option<PathBuf>,
    release: bool,
    optimized: bool,
) -> Result<PathBuf> {
    let total_start = Instant::now();
    let filename = script_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".into());
    let profile_label = if optimized {
        "optimized"
    } else if release {
        "release"
    } else {
        "debug"
    };
    eprintln!("{BOLD}        Rosy{RESET} v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("{BOLD}  Transpiling{RESET} {filename} ({profile_label})");

    // --- Step 1: Parse ---
    step(1, 5, "Parsing");
    let t = Instant::now();
    let script = std::fs::read_to_string(script_path).with_context(|| {
        format!(
            "Failed to read script file from `{}`!",
            script_path.display()
        )
    })?;
    let program = ast::CosyParser::parse(ast::Rule::program, &script)
        .context("Couldn't parse!")?
        .next()
        .context("Expected a program")?;
    step_done(t);

    // --- Step 2: AST Generation ---
    step(2, 5, "Building AST");
    let t = Instant::now();
    let mut ast = Program::from_rule(program)
        .context("Failed to build AST!")?
        .context("Expected a program")?;
    step_done(t);

    // --- Step 3: Type Resolution ---
    step(3, 5, "Resolving types");
    let t = Instant::now();
    let warnings = resolve::TypeResolver::resolve(&mut ast).context("Failed to resolve types!")?;
    step_done(t);
    for w in &warnings {
        eprintln!("{BOLD}{YELLOW}    warning{RESET}: {w}");
    }

    // --- Step 4: Transpilation ---
    step(4, 5, "Generating Rust code");
    let t = Instant::now();
    let TranspilationOutput { serialization, .. } = ast
        .transpile(&mut TranspilationInputContext::default())
        .map_err(|vec_errs| {
            step_fail();
            let mut combined = String::new();
            for (outer_ind, err) in vec_errs.iter().enumerate() {
                let mut body = String::new();
                for (ind, ctx) in err.chain().enumerate() {
                    body.push_str(&format!("  {}. {}\n", ind + 1, ctx));
                }
                combined.push_str(&format!(
                    "\n#{}: {}\nContext:\n{}",
                    outer_ind + 1,
                    err.root_cause(),
                    body
                ));
            }
            anyhow!(
                "Failed to transpile with the following errors:\n{}",
                combined
            )
        })?;

    // Detect whether the program uses MPI (only PLOOP generates rosy_mpi_context references)
    let uses_mpi = serialization.contains("rosy_mpi_context");
    if uses_mpi {
        info!("Program uses PLOOP — MPI support enabled in output");
    }

    // Determine output directory
    let rosy_output_path = output_dir.unwrap_or_else(|| PathBuf::from(".rosy_output"));

    // Create the output project structure from embedded templates
    embedded::create_output_project(&rosy_output_path, uses_mpi, optimized)
        .context("Failed to create output project structure")?;

    // Inject the transpiled code into main.rs
    let new_contents = embedded::inject_code(&serialization, uses_mpi)
        .context("Failed to inject transpiled code into template")?;

    write(rosy_output_path.join("src/main.rs"), &new_contents)
        .context("Failed to write Rust output file!")?;
    step_done(t);

    // --- Step 5: Compilation (piped to user's terminal) ---
    eprintln!("{BOLD}{CYAN}[5/5]{RESET} Compiling generated Rust code...");
    let mut cargo_args = vec!["build", "--bin", "rosy_output", "--color", "always"];
    if release {
        cargo_args.push("--release");
    }

    let status = Command::new("cargo")
        .args(&cargo_args)
        .current_dir(&rosy_output_path)
        .stdin(std::process::Stdio::null())
        .status()
        .context("Failed to spawn cargo build process")?;
    ensure!(
        status.success(),
        "Compilation failed with exit code: {:?}",
        status.code()
    );

    let build_profile = if release { "release" } else { "debug" };
    let binary_path = rosy_output_path.join(format!("target/{}/rosy_output", build_profile));

    let total_ms = total_start.elapsed().as_millis();
    eprintln!(
        "{BOLD}{GREEN}    Finished{RESET} in {DIM}{:.2}s{RESET}",
        total_ms as f64 / 1000.0
    );

    Ok(binary_path)
}

// ─── Construct Test Runner (`rosy test`) ────────────────────────────────────

/// Discover construct directories containing `test.rosy` under a base directory.
fn discover_construct_dirs(base: &std::path::Path) -> Vec<(String, PathBuf)> {
    let mut results = Vec::new();
    discover_construct_dirs_recursive(base, &mut results);
    results.sort_by(|a, b| a.0.cmp(&b.0));
    results
}

fn discover_construct_dirs_recursive(dir: &std::path::Path, results: &mut Vec<(String, PathBuf)>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            if path.join("test.rosy").is_file() {
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                results.push((name, path.clone()));
            }
            discover_construct_dirs_recursive(&path, results);
        }
    }
}

/// Extract the meaningful output from COSY's stdout.
fn extract_cosy_output(raw: &str) -> String {
    let mut after_exec = false;
    let mut lines = Vec::new();

    for line in raw.lines() {
        if after_exec {
            lines.push(line);
        } else if line.contains("BEGINNING EXECUTION") {
            after_exec = true;
        }
    }

    lines.join("\n")
}

/// Result of a single construct test.
#[derive(Debug)]
struct TestResult {
    label: String,
    ok: bool,
    elapsed_secs: f64,
    failure_msg: Option<String>,
}

/// Run a single construct test using the given build directory.
fn run_single_test(
    category: &str,
    name: &str,
    construct_path: &std::path::Path,
    build_dir: &std::path::Path,
    workspace_root: &std::path::Path,
    cosy_bin: Option<&std::path::Path>,
    release: bool,
) -> TestResult {
    let test_label = format!("{category}/{name}");
    let t = Instant::now();

    let rosy_script = construct_path.join("test.rosy");
    let fox_script = construct_path.join("test.fox");
    let rosy_output_path = construct_path.join("rosy_output.txt");
    let cosy_output_path = construct_path.join("cosy_output.txt");

    let mut cmd = Command::new("cargo");
    cmd.arg("run");
    if release {
        cmd.arg("--release");
    }
    cmd.arg("--manifest-path")
        .arg(workspace_root.join("Cargo.toml"))
        .arg("-p")
        .arg("rosy")
        .arg("--")
        .arg("run")
        .arg(&rosy_script)
        .arg("-d")
        .arg(build_dir)
        .current_dir(build_dir);

    let rosy_result = cmd.output();

    match rosy_result {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            if stdout.trim().is_empty() {
                return TestResult {
                    label: test_label,
                    ok: false,
                    elapsed_secs: t.elapsed().as_secs_f64(),
                    failure_msg: Some("empty output".to_string()),
                };
            }
            fs::write(&rosy_output_path, &stdout).ok();

            // Run COSY if available
            if let Some(cosy) = cosy_bin {
                if fox_script.exists() {
                    let child = Command::new(cosy)
                        .stdin(std::process::Stdio::piped())
                        .stdout(std::process::Stdio::piped())
                        .stderr(std::process::Stdio::piped())
                        .current_dir(construct_path)
                        .spawn();

                    if let Ok(mut child) = child {
                        {
                            use std::io::Write;
                            if let Some(mut stdin) = child.stdin.take() {
                                let _ = stdin.write_all(b"test\n");
                            }
                        }
                        if let Ok(cosy_result) = child.wait_with_output() {
                            let cosy_stdout =
                                String::from_utf8_lossy(&cosy_result.stdout).to_string();
                            let cosy_output = extract_cosy_output(&cosy_stdout);
                            if !cosy_output.trim().is_empty() {
                                fs::write(&cosy_output_path, &cosy_output).ok();
                            }
                        }
                    }
                }
            }

            TestResult {
                label: test_label,
                ok: true,
                elapsed_secs: t.elapsed().as_secs_f64(),
                failure_msg: None,
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            TestResult {
                label: test_label,
                ok: false,
                elapsed_secs: t.elapsed().as_secs_f64(),
                failure_msg: Some(format!("transpilation/execution failed\n{stderr}")),
            }
        }
        Err(e) => TestResult {
            label: test_label,
            ok: false,
            elapsed_secs: t.elapsed().as_secs_f64(),
            failure_msg: Some(format!("failed to spawn: {e}")),
        },
    }
}

/// Run all construct tests, printing results as they complete.
fn run_construct_tests(filter: Option<&str>, release: bool, parallel: usize) -> Result<()> {
    let parallel = parallel.max(1);
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_root
        .parent()
        .expect("Failed to get workspace root")
        .to_path_buf();

    let stmt_dir = crate_root.join("src/program/statements");
    let expr_dir = crate_root.join("src/program/expressions");

    let mut all_tests: Vec<(String, String, PathBuf)> = Vec::new();
    for (name, path) in discover_construct_dirs(&stmt_dir) {
        all_tests.push(("statements".to_string(), name, path));
    }
    for (name, path) in discover_construct_dirs(&expr_dir) {
        all_tests.push(("expressions".to_string(), name, path));
    }

    if let Some(f) = filter {
        all_tests.retain(|(_, name, _)| name.contains(f));
    }

    let total = all_tests.len();
    if total == 0 {
        eprintln!(
            "No tests found{}",
            filter
                .map(|f| format!(" matching '{f}'"))
                .unwrap_or_default()
        );
        return Ok(());
    }

    eprintln!(
        "{BOLD}        Rosy{RESET} v{} — testing {total} construct{}",
        env!("CARGO_PKG_VERSION"),
        if total == 1 { "" } else { "s" }
    );
    if release {
        eprintln!("        Mode: release");
    }
    if parallel > 1 {
        eprintln!("    Parallel: {parallel} workers");
    }

    let cosy_bin = workspace_root.join("cosy");
    let has_cosy = cosy_bin.exists() && cosy_bin.is_file();
    if has_cosy {
        eprintln!("        COSY: {GREEN}found{RESET}");
    }
    eprintln!();

    // Create build directories in tmp upfront
    let tmp_base = std::env::temp_dir().join(format!("rosy_test_{}", std::process::id()));
    let build_dirs: Vec<PathBuf> = (0..parallel)
        .map(|i| {
            let dir = tmp_base.join(format!("worker_{i}"));
            fs::create_dir_all(&dir).expect("Failed to create build directory");
            dir
        })
        .collect();

    eprintln!("  Build dirs: {}\n", tmp_base.display());

    let total_start = Instant::now();

    // Shared state for the work queue
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    };
    let work_index = Arc::new(AtomicUsize::new(0));
    let all_tests = Arc::new(all_tests);
    let results: Arc<Mutex<Vec<TestResult>>> = Arc::new(Mutex::new(Vec::with_capacity(total)));

    // Spawn worker threads
    let mut handles = Vec::new();
    for worker_id in 0..parallel {
        let work_index = Arc::clone(&work_index);
        let all_tests = Arc::clone(&all_tests);
        let results = Arc::clone(&results);
        let build_dir = build_dirs[worker_id].clone();
        let workspace_root = workspace_root.clone();
        let cosy_bin = if has_cosy {
            Some(cosy_bin.clone())
        } else {
            None
        };

        handles.push(std::thread::spawn(move || {
            loop {
                let i = work_index.fetch_add(1, Ordering::SeqCst);
                if i >= all_tests.len() {
                    break;
                }

                let (category, name, construct_path) = &all_tests[i];
                let result = run_single_test(
                    category,
                    name,
                    construct_path,
                    &build_dir,
                    &workspace_root,
                    cosy_bin.as_deref(),
                    release,
                );

                results.lock().unwrap().push(result);
            }
        }));
    }

    // Wait for all workers, printing results as they arrive
    let print_handle = std::thread::spawn({
        let results = Arc::clone(&results);
        move || {
            let mut printed = 0usize;
            while printed < total {
                std::thread::sleep(std::time::Duration::from_millis(50));
                let results = results.lock().unwrap();
                while printed < results.len() {
                    let r = &results[printed];
                    printed += 1;
                    if r.ok {
                        eprintln!(
                            "{DIM}[{:>3}/{}]{RESET} {}... {GREEN}ok{RESET} {DIM}({:.1}s){RESET}",
                            printed, total, r.label, r.elapsed_secs
                        );
                    } else {
                        eprintln!(
                            "{DIM}[{:>3}/{}]{RESET} {}... {RED}FAIL{RESET} {DIM}({:.1}s){RESET}",
                            printed, total, r.label, r.elapsed_secs
                        );
                    }
                }
            }
        }
    });

    for h in handles {
        h.join().expect("Worker thread panicked");
    }
    print_handle.join().expect("Print thread panicked");

    // Cleanup build directories
    let _ = fs::remove_dir_all(&tmp_base);

    // Summarize
    let all_results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
    let passed = all_results.iter().filter(|r| r.ok).count();
    let failed = all_results.iter().filter(|r| !r.ok).count();
    let total_secs = total_start.elapsed().as_secs_f64();

    eprintln!();

    let failures: Vec<&TestResult> = all_results.iter().filter(|r| !r.ok).collect();
    if !failures.is_empty() {
        eprintln!("{BOLD}{RED}failures:{RESET}\n");
        for f in &failures {
            eprintln!(
                "  {}: {}\n",
                f.label,
                f.failure_msg.as_deref().unwrap_or("unknown")
            );
        }
    }

    eprintln!(
        "{BOLD}test result:{RESET} {} passed, {} failed ({:.1}s)",
        passed, failed, total_secs
    );

    if failed > 0 {
        Err(anyhow!("{} test(s) failed", failed))
    } else {
        Ok(())
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Kick off a background version check (non-blocking)
    let update_handle = update_check::spawn_update_check();

    let cli = Cli::parse();

    // Handle Test command separately (no transpilation pipeline)
    if let Commands::Test {
        filter,
        release,
        parallel,
    } = &cli.command
    {
        update_handle.finish();
        return run_construct_tests(filter.as_deref(), *release, *parallel);
    }

    // Extract common fields and transpile
    let (source, output_dir, release, optimized, cosy_syntax, output_name) = match &cli.command {
        Commands::Run {
            source,
            output_dir,
            release,
            optimized,
            cosy_syntax,
        } => (
            source.clone(),
            output_dir.clone(),
            *release || *optimized,
            *optimized,
            *cosy_syntax,
            None,
        ),
        Commands::Build {
            source,
            output,
            output_dir,
            release,
            optimized,
            cosy_syntax,
        } => {
            let name = output.clone().unwrap_or_else(|| {
                source
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("rosy_output")
                    .to_string()
            });
            (
                source.clone(),
                output_dir.clone(),
                *release || *optimized,
                *optimized,
                *cosy_syntax,
                Some(name),
            )
        }
        Commands::Test { .. } => unreachable!(),
    };

    syntax_config::set_cosy_syntax(cosy_syntax);
    let binary_path = rosy(&source, output_dir, release, optimized)?;

    // Show update notice after transpilation (network has had time)
    update_handle.finish();

    // Run or copy the binary
    match cli.command {
        Commands::Run { .. } => {
            eprintln!("{BOLD}{CYAN}     Running{RESET} {}\n", source.display());

            let status = Command::new(&binary_path)
                .status()
                .with_context(|| format!("Failed to run binary at `{}`!", binary_path.display()))?;
            ensure!(
                status.success(),
                "Execution failed with exit code: {:?}",
                status.code()
            );
        }
        Commands::Build { .. } => {
            let destination = PathBuf::from(output_name.unwrap());
            std::fs::copy(&binary_path, &destination)
                .context("Failed to copy binary to current directory")?;
            eprintln!("  Binary written to {BOLD}{}{RESET}", destination.display());
        }
        Commands::Test { .. } => unreachable!(),
    }

    Ok(())
}
