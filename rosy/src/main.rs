mod transpile;
mod ast;
mod embedded;
#[allow(unused_imports, dead_code)]
mod rosy_lib;

use crate::{transpile::{TranspilationInputContext, TranspilationOutput, Transpile}, ast::build_ast};
use std::{fs::write, path::PathBuf, process::Command};
use anyhow::{ensure, Context, Result, anyhow};
use pest::Parser;
use tracing::info;
use tracing_subscriber;

fn rosy (
    script_path: &PathBuf,
    output_dir: Option<PathBuf>
) -> Result<()> {
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
    let output = Command::new("cargo")
        .args(&["build", "--release", "--bin", "rosy_output"])
        .current_dir(&rosy_output_path)
        .output()
        .context("Failed to spawn cargo build process")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    info!("Cargo stdout:\n{}", stdout);
    info!("Cargo stderr:\n{}", stderr);
    ensure!(output.status.success(), "Compilation failed with exit code: {:?} with stdout:\n{stdout} and stderr:\n{stderr}", output.status.code());

    let binary_path = rosy_output_path.join("target/release/rosy_output");
    info!("Build complete! Binary at: {}", binary_path.display());

    // Copy the binary to the current directory
    let destination = PathBuf::from("rosy_output");
    std::fs::copy(&binary_path, &destination)
        .context("Failed to copy binary to current directory")?;
    info!("Copied binary to: {}", destination.display());

    Ok(())
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

    let script_path = PathBuf::from(std::env::args()
        .nth(1)
        .unwrap_or("examples/basic.rosy".to_string()));

    // For now, use None for output_dir (will use temp directory)
    // In the future, we can add --output-dir flag here
    rosy(&script_path, None)
}