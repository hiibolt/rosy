use std::io::Write;

use pest::Parser;
use pest_derive::Parser;
use anyhow::{bail, Context, Result};

#[derive(Parser)]
#[grammar = "assets/grammars/cosy.pest"]
struct CosyParser;

#[derive(Debug)]
struct Program {
    statements: Vec<Statement>,
}

#[derive(Debug)]
enum Statement {
    VarDecl { name: String, length: u8 },
    Write { exprs: Vec<Expr> },
    Assign { name: String, value: Expr },
    Procedure { name: String, args: Vec<String>, body: Vec<Statement> },
}
impl TryFrom<Statement> for String {
    type Error = anyhow::Error;

    fn try_from(value: Statement) -> Result<Self> {
        match value {
            Statement::VarDecl { name, .. } => {
                Ok(format!("Cosy {};", name))
            },
            Statement::Write { exprs } => {
                let mut exprs_sts = Vec::new();
                for expr in exprs {
                    let expr_st: String = expr.try_into().unwrap();
                    exprs_sts.push(expr_st);
                }

                Ok(format!("std::cout << {} << std::endl;", exprs_sts.join(" << ")))
            }
            Statement::Assign { name, value } => {
                let value_st: String = value.try_into().unwrap();
                Ok(format!("{} = {};", name, value_st))
            },
            Statement::Procedure {
                name,
                args,
                body
            } => {
                let fn_type = if name == "RUN" { "int" } else { "void" };
                let fn_name = if name == "RUN" { "main" } else { &name };

                let mut body_sts = Vec::new();
                for stmt in body {
                    let mut stmt_st: String = stmt.try_into()
                        .context("Failed to convert statement to string!")?;
                    stmt_st.insert(0, '\t'); // Indent the body statements
                    body_sts.push(stmt_st);
                }

                Ok(format!("{} {}({}) {{\n{}\n}}", fn_type, fn_name, args.join(", "), body_sts.join("\n")))
            }
        }
    }
}

#[derive(Debug)]
enum Expr {
    Number(i32),
    Var(String),
    Exp { expr: Box<Expr> },
}
impl TryFrom<Expr> for String {
    type Error = anyhow::Error;

    fn try_from(value: Expr) -> Result<Self> {
        let res = match value {
            Expr::Number(n) => {
                format!("Cosy({})", n)
            },
            Expr::Var(name) => name.to_string(),
            Expr::Exp { expr } => {
                let sub_expr: String = (*expr).try_into()?;
                format!("Cosy(e^({}))", sub_expr)
            }
        };

        Ok(res)
    }
}

fn main() -> Result<()> {
    let script = std::fs::read_to_string("inputs/basic.cosy")
        .context("Failed to read script file!")?;

    // [ Parsing ]
    let program = CosyParser::parse(Rule::program, &script)
        .context("Couldn't parse!")?
        .next()
        .context("Expected a program")?;

    // [ AST Generation ]
    let ast = build_ast(program).context("Failed to build AST!")?;
    println!("{:#?}", ast);


    // [ Transpilation ]
    let cosy_lib = std::fs::read_to_string("assets/cosy_lib/cosy.cpp")
        .context("Failed to read cosy_lib.cpp!")?;
    let mut output = cosy_lib + "\n\n\n\n/// Automatically generated\n";

    for statement in ast.statements {
        let statement_st: String = statement.try_into()
            .context("Failed to convert statement to string!")?;
        output.push_str(&statement_st);
        output.push('\n');
    }

    let output_file = std::fs::File::create("outputs/main.cpp")
        .context("Failed to create output file!")?;
    let mut writer = std::io::BufWriter::new(output_file);
    writer.write_all(output.as_bytes())
        .context("Failed to write to output file!")?;

    Ok(())
}

fn build_statement (
    pair: pest::iterators::Pair<Rule>
) -> Result<Option<Statement>> {
    match pair.as_rule() {
        Rule::var_decl => {
            let mut inner = pair.into_inner();
            let name = inner.next()
                .context("Missing first token `variable_name`!")?
                .as_str().to_string();
            let length = inner.next()
                .context("Missing second token `variable_length`!")?
                .as_str().parse::<u8>()?;
            Ok(Some(Statement::VarDecl { name, length }))
        }
        Rule::write => {
            let mut inner = pair.into_inner();
            let mut exprs = Vec::new();
            while let Some(expr_pair) = inner.next() {
                if expr_pair.as_rule() == Rule::semicolon {
                    break;
                }

                let expr = build_expr(expr_pair)
                    .context("Failed to build expression in `write` statement!")?;
                exprs.push(expr);
            }
            Ok(Some(Statement::Write { exprs }))
        }
        Rule::assignment => {
            let mut inner = pair.into_inner();
            let name = inner.next()
                .context("Missing first token `variable_name`!")?
                .as_str().to_string();
            let expr_pair = inner.next()
                .context("Missing second token `expr`!")?;
            let expr = build_expr(expr_pair)?;
            Ok(Some(Statement::Assign { name, value: expr }))
        }
        Rule::procedure => {
            let mut inner = pair.into_inner();
            let (name, args) = {
                let mut start_procedure_inner = inner
                    .next()
                    .context("Missing first token `start_procedure`!")?
                    .into_inner();

                let name = start_procedure_inner.next()
                    .context("Missing procedure name!")?
                    .as_str().to_string();
                
                let mut args = Vec::new();
                // Collect all remaining arguments (procedure_argument_name tokens)
                while let Some(arg) = start_procedure_inner.next() {
                    if arg.as_rule() == Rule::semicolon {
                        break;
                    }

                    args.push(arg.as_str().to_string());
                }

                (name, args)
            };
            
            let body = {
                let mut statements = Vec::new();

                // Process remaining elements (statements and end_procedure)
                while let Some(element) = inner.next() {
                    // Skip the end_procedure element
                    if element.as_rule() == Rule::end_procedure {
                        break;
                    }
                    
                    let pair_input = element.as_str();
                    if let Some(stmt) = build_statement(element)
                        .with_context(|| format!("Failed to build statement from:\n{}", pair_input))? {
                        statements.push(stmt);
                    }
                }

                statements
            };

            Ok(Some(Statement::Procedure { name, args, body }))
        }
        // Ignored
        Rule::begin | Rule::end | Rule::EOI | Rule::end_procedure => Ok(None),
        other => bail!("Unexpected statement: {:?}", other),
    }
}

fn build_ast(pair: pest::iterators::Pair<Rule>) -> Result<Program> {
    let mut statements = Vec::new();

    for stmt in pair.into_inner() {
        let pair_input = stmt.as_str();
        if let Some(statement) = build_statement(stmt)
            .with_context(|| format!("Failed to build statement from:\n{}", pair_input))? {
            statements.push(statement);
        }
    }

    Ok(Program { statements })
}
fn build_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    match pair.as_rule() {
        Rule::number => {
            let n = pair.as_str().parse::<i32>()?;
            Ok(Expr::Number(n))
        }
        Rule::identifier => Ok(Expr::Var(pair.as_str().to_string())),
        Rule::exp => {
            let mut inner = pair.into_inner();
            let expr_pair = inner.next()
                .context("Missing inner expression for `EXP`!")?;
            let expr = Box::new(build_expr(expr_pair)?);
            Ok(Expr::Exp { expr })
        }
        _ => anyhow::bail!("Unexpected expr: {:?}", pair.as_rule()),
    }
}
