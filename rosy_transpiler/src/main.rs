mod transpile;
mod parsing;
mod ast;
mod analysis;

use std::{fs::write, process::Command};

use pest::Parser;
use anyhow::{ensure, Context, Result};
use colored::Colorize;

use crate::{ast::build_ast, parsing::{CosyParser, Rule}, transpile::Transpile, analysis::analyze_program};

fn main() -> Result<()> {
    // Stage 0 - Load script and delete + create outputs directory
    let script = std::fs::read_to_string("inputs/basic.cosy")
        .context("Failed to read script file!")?;
    if std::fs::metadata("outputs").is_ok() {
        std::fs::remove_dir_all("outputs")
            .context("Failed to remove outputs directory")?;
    }
    std::fs::create_dir("outputs")
        .context("Failed to create outputs directory")?;

    // Stage 1 - Parsing
    println!("{}", "Stage 1 - Parsing".bright_blue());
    let program = CosyParser::parse(Rule::program, &script)
        .context("Couldn't parse!")?
        .next()
        .context("Expected a program")?;

    // Stage 2 - AST Generation
    println!("{}", "Stage 2 - AST Generation".bright_green());
    let ast = build_ast(program)
        .context("Failed to build AST!")?;
    write("outputs/main.ast", &format!("{:#?}", ast))
        .context("Failed to write AST output file!")?;

    // Stage 3 - Static Analysis
    println!("{}", "Stage 3 - Static Analysis".bright_cyan());
    analyze_program(&ast)
        .context("Static analysis failed!")?;

    // Stage 4 - Transpilation
    println!("{}", "Stage 4 - Transpilation".bright_yellow());
    let rust = ast.transpile()
        .context("Failed to transpile AST to C++!")?;

    // Stage 4.5 - Write intermediaries to files for inspection
    std::fs::remove_dir_all("outputs")
        .context("Failed to remove outputs directory")?;
    let new_contents = {
        // Get the contents of `rosy_output/src/main.rs`
        let mainfile = std::fs::read_to_string("rosy_output/src/main.rs")
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
            rust.lines()
                .map(|line| format!("\t{}", line))
                .collect::<Vec<String>>()
                .join("\n"), 
            after_inject
        )
            
    };
    write("rosy_output/src/main.rs", &new_contents)
        .context("Failed to write Rust output file!")?;

    // Stage 5 - Compilation
    println!("{}", "Stage 5 - Compilation".bright_magenta());
    let mut child = Command::new("cargo")
        .args(&["build", "--release", "--bin", "rosy_output"])
        .spawn()?;
    let exit_status = child.wait()
        .context("Failed to wait for child process")?;
    ensure!(exit_status.success(), "Compilation failed with exit code: {:?}", exit_status.code());

    // Stage 6 - Execution
    println!("{}", "Stage 6 - Execution".bright_red());
    let exit_status = Command::new("target/release/rosy_output")
        .status()
        .context("Failed to execute generated binary")?;

    // Debug printing
    println!("{}", format!("Exit code: {:?}", exit_status.code()).bright_cyan());
    println!("{}", "Generated AST written to `outputs/main.ast`".bright_cyan());
    println!("{}", "Generated C++ written to `outputs/src/main.rs`".bright_cyan());
    println!("{}", "Generated binary written to `target/release/rosy_output`".bright_cyan());

    Ok(())
}