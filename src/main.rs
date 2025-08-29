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
    // Stage 0 - Load script and clean outputs directory
    let script = std::fs::read_to_string("inputs/correct_underscore_test.cosy")
        .context("Failed to read script file!")?;
    for entry in std::fs::read_dir("outputs")? {
        let entry = entry.context("Couldn't view directory entry!")?;
        if entry.file_type()
            .context("Couldn't check filetype!")?
            .is_file()
        {
            std::fs::remove_file(entry.path())
                .context("Couldn't remove file!")?;
        }
    }

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
    let mut child = Command::new("cp")
        .args(&["-r", "assets/rust", "outputs"])
        .spawn()
        .context("Failed to spawn child process")?;
    let exit_status = child.wait()
        .context("Failed to wait for child process")?;
    ensure!(exit_status.success(), "Copying assets failed with exit code: {:?}", exit_status.code());
    write("outputs/src/main.rs", &rust)
        .context("Failed to write Rust output file!")?;

    // Stage 5 - Compilation
    println!("{}", "Stage 5 - Compilation".bright_magenta());
    let mut child = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir("outputs")
        .spawn()?;
    let exit_status = child.wait()
        .context("Failed to wait for child process")?;
    ensure!(exit_status.success(), "Compilation failed with exit code: {:?}", exit_status.code());

    // Stage 6 - Execution
    println!("{}", "Stage 6 - Execution".bright_red());
    let exit_status = Command::new("outputs/target/release/rust")
        .status()
        .context("Failed to execute generated binary")?;

    // Debug printing
    println!("{}", format!("Exit code: {:?}", exit_status.code()).bright_cyan());
    println!("{}", "Generated AST written to `outputs/main.ast`".bright_cyan());
    println!("{}", "Generated C++ written to `outputs/src/main.rs`".bright_cyan());
    println!("{}", "Generated binary written to `outputs/target/release/rust`".bright_cyan());

    Ok(())
}