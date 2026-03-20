//! # Rosy Transpiler
//!
//! A modern Rust-based programming language based on the COSY INFINITY syntax,
//! designed for scientific computing and beam physics applications.
//!
//! ## Quick Start
//!
//! ```bash
//! # Run a Rosy script directly
//! rosy run examples/basic.rosy
//!
//! # Build a standalone binary
//! rosy build examples/basic.rosy
//!
//! # Build with optimizations
//! rosy build examples/basic.rosy --release
//! ```
//!
//! ## Rosy Language Example
//!
//! ```text
//! BEGIN;
//!     FUNCTION (RE) ADD_TWO a (RE) b (RE);
//!         ADD_TWO := a + b;
//!     ENDFUNCTION;
//!
//!     PROCEDURE RUN;
//!         VARIABLE (RE) x;
//!         VARIABLE (RE) y;
//!         x := 3;
//!         y := 4;
//!         WRITE 6 "Result: " ST(ADD_TWO(x, y));
//!     ENDPROCEDURE;
//!
//!     RUN;
//! END;
//! ```
//!
//! ## Type System
//!
//! Rosy supports the following base types:
//!
//! | Type | Description | Rust Equivalent |
//! |------|-----------|----------------|
//! | `RE` | Real number | `f64` |
//! | `ST` | String | `String` |
//! | `LO` | Logical (boolean) | `bool` |
//! | `CM` | Complex number | `Complex64` |
//! | `VE` | Vector of reals | `Vec<f64>` |
//! | `DA` | Differential Algebra (Taylor series) | `DA` |
//! | `CD` | Complex Differential Algebra | `CD` |
//!
//! Multi-dimensional arrays are supported: `(RE 2 2)` is a 2x2 VE of reals.
//!
//! ## Ways to Learn Rosy
//! ### View All Language Features and Documentation
//! - **[`program`]** — The AST structure: expressions and statements
//!   - [`program::expressions`] — All expression types (operators, functions, literals, etc.)
//!   - [`program::statements`] — All statement types (control flow, I/O, declarations, etc.)
//! ### View Example Rosy Programs
//! - **[Link to GitHub](https://github.com/hiibolt/rosy/tree/master/examples)** — A collection of example Rosy scripts demonstrating various features and use cases
//!
//! ## IDE Support
//!
//! A VSCode extension for Rosy syntax highlighting is available. To generate and install it:
//!
//! 1. Generate the extension:
//!    ```bash
//!    cargo run --bin generate_vscode_extension
//!    ```
//! 2. Copy the `rosy-vscode-extension/` folder to your VSCode extensions directory:
//!    - **Linux/macOS**: `~/.vscode/extensions/`
//!    - **Windows**: `%USERPROFILE%\.vscode\extensions\`
//! 3. Reload VSCode — any `.rosy` or `.fox` file will have syntax highlighting
//!
//! ## Building the Docs
//!
//! ```bash
//! cargo doc --document-private-items --no-deps --open
//! ```

mod ast;
mod embedded;
mod program;
mod resolve;
#[allow(unused_imports, dead_code)]
mod rosy_lib;
mod syntax_config;
mod transpile;

use crate::{ast::FromRule, program::Program, transpile::*};
use anyhow::{Context, Result, anyhow, ensure};
use clap::{Parser as ClapParser, Subcommand};
use pest::Parser;
use std::{fs::write, path::PathBuf, process::Command};
use tracing::info;
use tracing_subscriber;

/// Rosy Transpiler - Converts Rosy source code to executable Rust programs
#[derive(ClapParser)]
#[command(name = "rosy")]
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
    info!("Loading script...");
    let script = std::fs::read_to_string(&script_path).with_context(|| {
        format!(
            "Failed to read script file from `{}`!",
            script_path.display()
        )
    })?;

    info!("Stage 1 - Parsing");
    let program = ast::CosyParser::parse(ast::Rule::program, &script)
        .context("Couldn't parse!")?
        .next()
        .context("Expected a program")?;

    info!("Stage 2 - AST Generation");
    let mut ast = Program::from_rule(program)
        .context("Failed to build AST!")?
        .context("Expected a program")?;

    info!("Stage 2.5 - Type Resolution");
    resolve::TypeResolver::resolve(&mut ast).context("Failed to resolve types!")?;

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

    info!("Creating output project at: {}", rosy_output_path.display());

    // Create the output project structure from embedded templates
    embedded::create_output_project(&rosy_output_path, uses_mpi)
        .context("Failed to create output project structure")?;

    // Inject the transpiled code into main.rs
    let new_contents = embedded::inject_code(&serialization, uses_mpi)
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
    ensure!(
        output.status.success(),
        "Compilation failed with exit code: {:?} with stdout:\n{stdout} and stderr:\n{stderr}",
        output.status.code()
    );

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
        Commands::Run {
            source,
            output_dir,
            release,
            cosy_syntax,
        } => {
            syntax_config::set_cosy_syntax(cosy_syntax);
            info!("Running Rosy script: {}", source.display());
            let binary_path = rosy(&source, output_dir, release)?;
            info!(
                "Compilation successful! Binary remains at: {}",
                binary_path.display()
            );

            // Run the binary, piping stdout and stderr
            let mut run_command = Command::new(&binary_path);
            let status = run_command
                .status()
                .with_context(|| format!("Failed to run binary at `{}`!", binary_path.display()))?;
            ensure!(
                status.success(),
                "Execution failed with exit code: {:?}",
                status.code()
            );
        }

        Commands::Build {
            source,
            output,
            output_dir,
            release,
            cosy_syntax,
        } => {
            syntax_config::set_cosy_syntax(cosy_syntax);
            info!("Building Rosy script: {}", source.display());
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
