mod transpile;
mod ast;
mod embedded;
mod statements;
#[allow(unused_imports, dead_code)]
mod rosy_lib;

use crate::{transpile::{TranspilationInputContext, TranspilationOutput, Transpile}, ast::build_ast};
use std::{fs::write, path::PathBuf, process::Command};
use anyhow::{ensure, Context, Result, anyhow};
use pest::Parser;
use tracing::info;
use tracing_subscriber;
use clap::{Parser as ClapParser, Subcommand};

/// ROSY Transpiler - Converts ROSY source code to executable Rust programs
#[derive(ClapParser)]
#[command(name = "rosy")]
#[command(about = "ROSY Transpiler for beam physics calculations", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a ROSY script directly without copying binary to PWD
    Run {
        /// Path to the ROSY source file
        source: PathBuf,
        
        /// Output directory for build artifacts (default: .rosy_output)
        #[arg(short = 'd', long)]
        output_dir: Option<PathBuf>,
        
        /// Build in release mode with optimizations
        #[arg(short, long)]
        release: bool,
    },
    
    /// Build a ROSY script and place the binary in PWD
    Build {
        /// Path to the ROSY source file
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
    },
}

fn rosy (
    script_path: &PathBuf,
    output_dir: Option<PathBuf>,
    release: bool,
) -> Result<PathBuf> {
    info!("Loading script...");
    let script = std::fs::read_to_string(&script_path)
        .with_context(|| format!("Failed to read script file from `{}`!", script_path.display()))?;

    info!("Stage 1 - Parsing");
    let program = ast::CosyParser::parse(ast::Rule::program, &script)
        .context("Couldn't parse!")?
        .next()
        .context("Expected a program")?;

    info!("Stage 2 - AST Generation");
    let ast = build_ast(program)
        .context("Failed to build AST!")?;

    info!("Stage 3 - Transpilation");
    let TranspilationOutput { serialization, .. } = ast
        .transpile(&mut TranspilationInputContext::default())
        .map_err(|vec_errs| {
            let mut combined = String::new();
            for (outer_ind, err) in vec_errs.iter().enumerate() {
                let mut body = String::new();
                for (ind, ctx) in err.chain().enumerate() {
                    body.push_str(&format!("  {}. {}\n", ind + 1, ctx));
                }
                combined.push_str(&format!("\n#{}: {}\nContext:\n{}", outer_ind + 1, err.root_cause(), body));
            }
            anyhow!("Failed to transpile with the following errors:\n{}", combined)
        })?;

    // Determine output directory
    let rosy_output_path = output_dir.unwrap_or_else(|| {
        PathBuf::from(".rosy_output")
    });
    
    info!("Creating output project at: {}", rosy_output_path.display());
    
    // Create the output project structure from embedded templates
    embedded::create_output_project(&rosy_output_path)
        .context("Failed to create output project structure")?;
    
    // Inject the transpiled code into main.rs
    let new_contents = embedded::inject_code(&serialization)
        .context("Failed to inject transpiled code into template")?;
    
    write(rosy_output_path.join("src/main.rs"), &new_contents)
        .context("Failed to write Rust output file!")?;

    info!("Stage 4 - Compilation");
    // We ensure to collect the output and emit it
    //  via `info!` so that if there are any
    //  compilation errors, they are visible
    //  in the logs.
    let mut cargo_args = vec!["build", "--bin", "rosy_output"];
    if release {
        cargo_args.push("--release");
    }
    
    let output = Command::new("cargo")
        .args(&cargo_args)
        .current_dir(&rosy_output_path)
        .output()
        .context("Failed to spawn cargo build process")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    info!("Cargo stdout:\n{}", stdout);
    info!("Cargo stderr:\n{}", stderr);
    ensure!(output.status.success(), "Compilation failed with exit code: {:?} with stdout:\n{stdout} and stderr:\n{stderr}", output.status.code());

    let build_profile = if release { "release" } else { "debug" };
    let binary_path = rosy_output_path.join(format!("target/{}/rosy_output", build_profile));
    info!("Build complete! Binary at: {}", binary_path.display());

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

    let cli = Cli::parse();

    match cli.command {
        Commands::Run { source, output_dir, release } => {
            info!("Running ROSY script: {}", source.display());
            let binary_path = rosy(&source, output_dir, release)?;
            info!("Compilation successful! Binary remains at: {}", binary_path.display());
            
            // Run the binary, piping stdout and stderr
            let mut run_command = Command::new(&binary_path);
            let status = run_command
                .status()
                .with_context(|| format!("Failed to run binary at `{}`!", binary_path.display()))?;
            ensure!(status.success(), "Execution failed with exit code: {:?}", status.code());
        }
        
        Commands::Build { source, output, output_dir, release } => {
            info!("Building ROSY script: {}", source.display());
            let binary_path = rosy(&source, output_dir, release)?;
            
            // Determine output name
            let output_name = output.unwrap_or_else(|| {
                source
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("rosy_output")
                    .to_string()
            });
            
            // Copy binary to PWD
            let destination = PathBuf::from(&output_name);
            std::fs::copy(&binary_path, &destination)
                .context("Failed to copy binary to current directory")?;
            info!("Binary copied to: {}", destination.display());
        }
    }

    Ok(())
}