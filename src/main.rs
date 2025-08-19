mod transpile;
mod parsing;
mod ast;

use std::io::Write;

use pest::Parser;
use anyhow::{Context, Result};

use crate::{ast::build_ast, parsing::{CosyParser, Rule}, transpile::Transpile};

fn main() -> Result<()> {
    let script = std::fs::read_to_string("inputs/basic.cosy")
        .context("Failed to read script file!")?;

    // Stage 1 - Parsing
    let program = CosyParser::parse(Rule::program, &script)
        .context("Couldn't parse!")?
        .next()
        .context("Expected a program")?;

    // Stage 2 - AST Generation
    // Generate AST
    let ast = build_ast(program).context("Failed to build AST!")?;

    // Write to file
    let ast_output_file = std::fs::File::create("outputs/main.ast")
        .context("Failed to create AST output file!")?;
    let mut ast_writer = std::io::BufWriter::new(ast_output_file);
    ast_writer.write_all(format!("{:#?}", ast).as_bytes())
        .context("Failed to write AST to output file!")?;
    ast_writer.flush()
        .context("Failed to flush AST output file!")?;

    println!("Generated AST:\n{:#?}", ast);


    // Stage 3 - Transpilation
    // Load cosy_lib
    let cosy_lib = std::fs::read_to_string("assets/cosy_lib/cosy.cpp")
        .context("Failed to read cosy_lib.cpp!")?;
    let mut output = cosy_lib + "\n\n\n\n/// Automatically generated\n";

    // Transpile the AST to C++
    for statement in ast.statements {
        let statement_st: String = statement.transpile()
            .context("Failed to convert statement to string!")?;
        output.push_str(&statement_st);
        output.push('\n');
    }

    // Write to file
    let cpp_output_file = std::fs::File::create("outputs/main.cpp")
        .context("Failed to create output file!")?;
    let mut cpp_writer = std::io::BufWriter::new(cpp_output_file);
    cpp_writer.write_all(output.as_bytes())
        .context("Failed to write to output file!")?;
    cpp_writer.flush()
        .context("Failed to flush output file!")?;

    println!("Generated C++\n{}", output);

    Ok(())
}