mod transpile;
mod parsing;
mod ast;
mod analysis;

use std::{fs::write, io::Write, process::{Command, Stdio}};

use pest::Parser;
use anyhow::{ensure, Context, Result};
use colored::Colorize;

use crate::{ast::build_ast, parsing::{CosyParser, Rule}, transpile::Transpile, analysis::analyze_program};

fn main() -> Result<()> {
    // Stage 0 - Load script and clean outputs directory
    let script = std::fs::read_to_string("inputs/basic.cosy")
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

    // Stage 3 - Static Analysis
    println!("{}", "Stage 3 - Static Analysis".bright_cyan());
    analyze_program(&ast)
        .context("Static analysis failed!")?;

    // Stage 4 - Transpilation
    println!("{}", "Stage 4 - Transpilation".bright_yellow());
    let cpp = ast.transpile()
        .context("Failed to transpile AST to C++!")?;

    // Stage 5 - Compilation
    println!("{}", "Stage 5 - Compilation".bright_magenta());
    let mut child = Command::new("g++")
        .args(&["-x", "c++", "-", "-o", "outputs/main"])
        .stdin(Stdio::piped())
        .spawn()?;
    child.stdin.as_mut()
        .context("Failed to open stdin")?
        .write_all(cpp.as_bytes())
        .context("Failed to write to stdin")?;
    let exit_status = child.wait()
        .context("Failed to wait for child process")?;
    ensure!(exit_status.success(), "Compilation failed with exit code: {:?}", exit_status.code());

    // Stage 6 - Execution
    println!("{}", "Stage 6 - Execution".bright_red());
    let exit_status = Command::new("outputs/main")
        .status()
        .context("Failed to execute generated binary")?;

    // Write intermediaries to files for inspection
    write("outputs/main.ast", &format!("{:#?}", ast))
        .context("Failed to write AST output file!")?;
    write("outputs/main.cpp", &cpp)
        .context("Failed to write C++ output file!")?;

    // Debug printing
    println!("{}", format!("Exit code: {:?}", exit_status.code()).bright_cyan());
    println!("{}", "Generated AST written to `outputs/main.ast`".bright_cyan());
    println!("{}", "Generated C++ written to `outputs/main.cpp`".bright_cyan());

    Ok(())
}