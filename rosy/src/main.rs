//! # Rosy
//!
#![doc = concat!("**Version:** `v", env!("CARGO_PKG_VERSION"), "` — [Changelog](https://github.com/hiibolt/rosy/releases)")]
//!
//! A modern transpiler for the ROSY scientific programming language,
//! designed for beam physics and differential algebra applications.
//! ROSY programs are transpiled into self-contained, native Rust executables.
//!
//! ## Quick Start
//!
//! ```bash
//! rosy run examples/basic.rosy          # run a script
//! rosy build examples/basic.rosy -o out # build a binary
//! ```
//!
//! ## Language Reference
//!
//! Jump directly to what you need:
//!
//! ### Statements — things that *do* something
//!
//! | I want to... | Syntax | Link |
//! |--------------|--------|------|
//! | Declare a variable | `VARIABLE (RE) x;` | [`statements::core::var_decl`](program::statements::core::var_decl) |
//! | Assign a value | `x := expr;` | [`statements::core::assign`](program::statements::core::assign) |
//! | Branch with if/else | `IF cond; ... ENDIF;` | [`statements::core::if`](program::statements::core::r#if) |
//! | Loop (counted) | `LOOP i 1 10; ... ENDLOOP;` | [`statements::core::loop`](program::statements::core::r#loop) |
//! | Loop (conditional) | `WHILE cond; ... ENDWHILE;` | [`statements::core::while_loop`](program::statements::core::while_loop) |
//! | Loop (MPI parallel) | `PLOOP ... ENDPLOOP;` | [`statements::core::ploop`](program::statements::core::ploop) |
//! | Define a function | `FUNCTION (RE) F x (RE); ... ENDFUNCTION;` | [`statements::core::function`](program::statements::core::function) |
//! | Define a procedure | `PROCEDURE P; ... ENDPROCEDURE;` | [`statements::core::procedure`](program::statements::core::procedure) |
//! | Print output | `WRITE 6 'hello';` | [`statements::io::write`](program::statements::io::write) |
//! | Read input | `READ 5 x;` | [`statements::io::read`](program::statements::io::read) |
//! | Work with files | `OPENF`, `CLOSEF` | [`statements::io`](program::statements::io) |
//! | Initialize DA | `OV 5 3;` | [`statements::da::da_init`](program::statements::da::da_init) |
//! | Print DA values | `DAPRV`, `DAREV` | [`statements::da`](program::statements::da) |
//! | Optimize (FIT) | `FIT ... ENDFIT;` | [`statements::math::fit`](program::statements::math::fit) |
//!
//! ### Operators
//!
//! | Operator | Symbol | Link |
//! |----------|--------|------|
//! | Arithmetic | `+`, `-`, `*`, `/` | [`operators::arithmetic`](program::expressions::operators::arithmetic) |
//! | Comparison | `=`, `<>`, `<`, `>`, `<=`, `>=` | [`operators::comparison`](program::expressions::operators::comparison) |
//! | Unary | `-x`, `NOT x` | [`operators::unary`](program::expressions::operators::unary) |
//! | Collection | `&` (concat), `\|` (extract), `%` (derive) | [`operators::collection`](program::expressions::operators::collection) |
//! | Power | `^` | [`exponential::pow`](program::expressions::functions::math::exponential::pow) |
//!
//! ### Built-in Functions
//!
//! | Category | Functions | Link |
//! |----------|-----------|------|
//! | Trigonometry | `SIN`, `COS`, `TAN`, `ASIN`, `ACOS`, `ATAN`, `SINH`, `COSH`, `TANH` | [`math::trig`](program::expressions::functions::math::trig) |
//! | Exponential | `EXP`, `LOG`, `SQR`, `SQRT` | [`math::exponential`](program::expressions::functions::math::exponential) |
//! | Complex | `CMPLX`, `CONJ`, `REAL`, `IMAG` | [`math::complex`](program::expressions::functions::math::complex) |
//! | Rounding | `ABS`, `INT`, `NINT`, `NORM`, `CONS` | [`math::rounding`](program::expressions::functions::math::rounding) |
//! | Vector | `VMIN`, `VMAX` | [`math::vector`](program::expressions::functions::math::vector) |
//! | Query | `TYPE`, `ISRT`, `ISRT3` | [`math::query`](program::expressions::functions::math::query) |
//! | Type conversion | `ST()`, `CM()`, `RE()`, `LO()`, `VE()` | [`conversion`](program::expressions::functions::conversion) |
//! | String/utility | `LENGTH`, `TRIM`, `LTRIM` | [`sys`](program::expressions::functions::sys) |
//!
//! ### Literals
//!
//! | Type | Example | Link |
//! |------|---------|------|
//! | `RE` (real) | `3.14`, `42` | [`types::number`](program::expressions::types::number) |
//! | `ST` (string) | `'hello'` | [`types::string`](program::expressions::types::string) |
//! | `LO` (boolean) | `TRUE`, `FALSE` | [`types::boolean`](program::expressions::types::boolean) |
//! | `DA` | `DA(1)` | [`types::da`](program::expressions::types::da) |
//! | `CD` | `CD(1)` | [`types::cd`](program::expressions::types::cd) |
//!
//! ## Type System
//!
//! | Type | Description |
//! |------|-------------|
//! | `RE` | Real number (64-bit float) |
//! | `ST` | String |
//! | `LO` | Logical (boolean) |
//! | `CM` | Complex number |
//! | `VE` | Vector of reals |
//! | `DA` | Differential Algebra (Taylor series) |
//! | `CD` | Complex Differential Algebra |
//!
//! Multi-dimensional arrays are supported: `(RE 2 2)` creates a 2×2 matrix of reals.
//!
//! ## Example Program
//!
//! ```text
//! BEGIN;
//!     VARIABLE (RE) x;
//!     VARIABLE (RE) y;
//!     x := 3;
//!     y := SIN(x) + 1;
//!     WRITE 6 'y = ' ST(y);
//! END;
//! ```
//!
//! ## More Resources
//!
//! - **[Example programs](https://github.com/hiibolt/rosy/tree/master/examples)** on GitHub
//! - **[Installation & usage](https://github.com/hiibolt/rosy)** in the README

mod ast;
mod embedded;
mod program;
mod resolve;
#[allow(unused_imports, dead_code)]
mod rosy_lib;
mod syntax_config;
mod transpile;
mod update_check;

use crate::{ast::FromRule, program::Program, transpile::*};
use anyhow::{Context, Result, anyhow, ensure};
use clap::{Parser as ClapParser, Subcommand};
use pest::Parser;
use std::{fs::write, path::PathBuf, process::Command, time::Instant};
use tracing::info;
use tracing_subscriber;

// ANSI color helpers (stderr only)
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
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

        /// Enforce COSY INFINITY syntax: memory sizes are required in VARIABLE declarations
        #[arg(long)]
        cosy_syntax: bool,
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

        /// Enforce COSY INFINITY syntax: memory sizes are required in VARIABLE declarations
        #[arg(long)]
        cosy_syntax: bool,
    },
}

fn rosy(script_path: &PathBuf, output_dir: Option<PathBuf>, release: bool) -> Result<PathBuf> {
    let total_start = Instant::now();
    let filename = script_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".into());
    eprintln!(
        "{BOLD}  Transpiling{RESET} {filename} ({})",
        if release { "release" } else { "debug" }
    );

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
    resolve::TypeResolver::resolve(&mut ast).context("Failed to resolve types!")?;
    step_done(t);

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
    embedded::create_output_project(&rosy_output_path, uses_mpi)
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

    // Extract common fields and transpile
    let (source, output_dir, release, cosy_syntax, output_name) = match &cli.command {
        Commands::Run { source, output_dir, release, cosy_syntax } => {
            (source.clone(), output_dir.clone(), *release, *cosy_syntax, None)
        }
        Commands::Build { source, output, output_dir, release, cosy_syntax } => {
            let name = output.clone().unwrap_or_else(|| {
                source.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("rosy_output")
                    .to_string()
            });
            (source.clone(), output_dir.clone(), *release, *cosy_syntax, Some(name))
        }
    };

    syntax_config::set_cosy_syntax(cosy_syntax);
    let binary_path = rosy(&source, output_dir, release)?;

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
            eprintln!(
                "  Binary written to {BOLD}{}{RESET}",
                destination.display()
            );
        }
    }

    Ok(())
}
