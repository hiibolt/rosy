mod transpile;
mod ast;

use crate::{transpile::{TranspilationInputContext, TranspilationOutput, Transpile}, ast::build_ast};
use std::{fs::write, path::PathBuf, process::Command};
use anyhow::{ensure, Context, Result, anyhow};
use pest::Parser;
use tracing::info;
use tracing_subscriber;

fn rosy (
    root: &PathBuf,
    script_path: &PathBuf
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

    let rosy_output_path = root.join("rosy_output");
    let new_contents = {
        // Get the contents of `rosy_output/src/main.rs`
        let mainfile = std::fs::read_to_string(rosy_output_path.join("src/main.rs"))
            .context("Failed to read main.rs template file!")?;

        // Split by 
        //  `// <INJECT_START>`
        //  ...and...
        //  `// <INJECT_END>`
        //  ...removing what's in between and replacing 
        //  it with the generated Rust code
        let parts: Vec<&str> = mainfile.split("// <INJECT_START>").collect();
        ensure!(parts.len() == 2, "Expected exactly one '// <INJECT_START>' in main.rs template file!");
        let before_inject = parts[0];
        let parts: Vec<&str> = parts[1].split("// <INJECT_END>").collect();
        ensure!(parts.len() == 2, "Expected exactly one '// <INJECT_END>' in main.rs template file!");
        let after_inject = parts[1];

        format!("{}// <INJECT_START>\n{}\n\t// <INJECT_END>{}", 
            before_inject, 
            serialization.lines()
                .map(|line| format!("\t{}", line))
                .collect::<Vec<String>>()
                .join("\n"), 
            after_inject
        )
    };
    write(rosy_output_path.join("src/main.rs"), &new_contents)
        .context("Failed to write Rust output file!")?;

    info!("Stage 4 - Compilation");
    // We ensure to collect the output and emit it
    //  via `info!` so that if there are any
    //  compilation errors, they are visible
    //  in the logs.
    let output = Command::new("cargo")
        .args(&["build", "--release", "--bin", "rosy_output"])
        .current_dir(&root)
        .output()
        .context("Failed to spawn cargo build process")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    info!("Cargo stdout:\n{}", stdout);
    info!("Cargo stderr:\n{}", stderr);
    ensure!(output.status.success(), "Compilation failed with exit code: {:?} with stdout:\n{stdout} and stderr:\n{stderr}", output.status.code());

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

    let root_path = PathBuf::from(".");
    let script_path = PathBuf::from(std::env::args()
        .nth(1)
        .unwrap_or("examples/basic.rosy".to_string()));

    rosy(&root_path, &script_path)
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial_test::serial]
    fn basic_rosy() -> Result<()> {
        let root_path = PathBuf::from("..");
        let script_path = PathBuf::from("../examples/basic.rosy");

        let output = rosy(&root_path, &script_path)?;

        assert_eq!(
            output,
            concat!(
                "X: 3\n",
                "Y: 4\n",
                "Summation of 3 and 4: 7\n",
                "[1, 2, 3, 4, 5, 6, 7, 8]\n",
                "(2 + 1i)\n",
                "0\n",
                "2\n",
                "4\n",
                ""
            )
        );

        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn if_statements() -> Result<()> {
        let root_path = PathBuf::from("..");
        let script_path = PathBuf::from("../examples/if_statements.rosy");

        let output = rosy(&root_path, &script_path)?;

        assert_eq!(
            output,
            concat!(
                "First condition is true\n",
                "This should print - ELSEIF works!\n",
                "ELSE clause works!\n",
                "Number is: 42\n"
            )
        );

        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn global_variables() -> Result<()> {
        let root_path = PathBuf::from("..");
        let script_path = PathBuf::from("../examples/global_vars.rosy");

        let output = rosy(&root_path, &script_path)?;

        assert_eq!(
            output,
            concat!(
                "Initial state:\n",
                "=== Current Global State ===\n",
                "Counter: 0\n",
                "Message: Initial message\n",
                "Status: INCOMPLETE\n",
                "========================\n",
                "Counter incremented to: 1\n",
                "Counter incremented to: 2\n",
                "Counter incremented to: 3\n",
                "Global message set to: Hello from global variables!\n",
                "Operation marked as complete\n",
                "Final state:\n",
                "=== Current Global State ===\n",
                "Counter: 3\n",
                "Message: Hello from global variables!\n",
                "Status: COMPLETE\n",
                "========================\n",
                "Operation is complete!\n",
            )
        );

        Ok(())
    }
} */