mod transpile;
mod parsing;
mod ast;

use std::{fs::write, io::Write, process::{Command, Stdio}};

use pest::Parser;
use anyhow::{Context, Result};
use colored::Colorize;

use crate::{ast::build_ast, parsing::{CosyParser, Rule}, transpile::Transpile};

fn main() -> Result<()> {
    let script = std::fs::read_to_string("inputs/basic.cosy")
        .context("Failed to read script file!")?;

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

    // Stage 3 - Transpilation
    println!("{}", "Stage 3 - Transpilation".bright_yellow());
    let cpp = ast.transpile()
        .context("Failed to transpile AST to C++!")?;

    // Stage 4 - Compilation
    println!("{}", "Stage 4 - Compilation".bright_magenta());
    let mut child = Command::new("g++")
        .args(&["-x", "c++", "-", "-o", "outputs/main"])
        .stdin(Stdio::piped())
        .spawn()?;
    child.stdin.as_mut()
        .context("Failed to open stdin")?
        .write_all(cpp.as_bytes())
        .context("Failed to write to stdin")?;
    child.wait()
        .context("Failed to wait for child process")?;

    // Stage 5 - Execution
    println!("{}", "Stage 5 - Execution".bright_red());
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