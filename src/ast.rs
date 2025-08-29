use crate::parsing::{Rule, PRATT_PARSER};
use anyhow::{Result, Context, bail};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    VarDecl { name: String, _length: u8 },
    Write { exprs: Vec<Expr> },
    Assign { name: String, value: Expr },
    Procedure { name: String, args: Vec<String>, body: Vec<Statement> },
    ProcedureCall { name: String, args: Vec<Expr> },
    Function { name: String, args: Vec<String>, body: Vec<Statement> },
    FunctionCall { name: String, args: Vec<Expr> },
}


#[derive(Debug)]
pub enum Expr {
    Number(i32),
    String(String),
    Var(String),
    Exp { expr: Box<Expr> },
    Complex { expr: Box<Expr> },
    Add { left: Box<Expr>, right: Box<Expr> },
    Concat { terms: Vec<Box<Expr>> },
    FunctionCall { name: String, args: Vec<Expr> },
}

pub fn build_ast(pair: pest::iterators::Pair<Rule>) -> Result<Program> {
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

fn build_statement (
    pair: pest::iterators::Pair<Rule>
) -> Result<Option<Statement>> {
    match pair.as_rule() {
        Rule::var_decl => {
            let mut inner = pair.into_inner();
            let name = inner.next()
                .context("Missing first token `variable_name`!")?
                .as_str().to_string();
            let _length = inner.next()
                .context("Missing second token `variable_length`!")?
                .as_str().parse::<u8>()?;
            Ok(Some(Statement::VarDecl { name, _length }))
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
        },
        Rule::procedure_call => {
            let mut inner = pair.into_inner();
            let name = inner.next()
                .context("Missing procedure name in procedure call!")?
                .as_str().to_string();
            
            let mut args = Vec::new();
            // Collect all remaining arguments (expressions)
            while let Some(arg_pair) = inner.next() {
                if arg_pair.as_rule() == Rule::semicolon {
                    break;
                }
                
                let expr = build_expr(arg_pair)
                    .context("Failed to build expression in procedure call!")?;
                args.push(expr);
            }
            
            Ok(Some(Statement::ProcedureCall { name, args }))
        },
        Rule::function => {
            let mut inner = pair.into_inner();
            let (name, args) = {
                let mut start_function_inner = inner
                    .next()
                    .context("Missing first token `start_function`!")?
                    .into_inner();

                let name = start_function_inner.next()
                    .context("Missing function name!")?
                    .as_str().to_string();

                let mut args = Vec::new();
                // Collect all remaining arguments (function_argument_name tokens)
                while let Some(arg) = start_function_inner.next() {
                    if arg.as_rule() == Rule::semicolon {
                        break;
                    }

                    args.push(arg.as_str().to_string());
                }

                (name, args)
            };

            let body = {
                let mut statements = vec!(
                    Statement::VarDecl { name: name.clone(), _length: 8 }
                );

                // Process remaining elements (statements and end_function)
                while let Some(element) = inner.next() {
                    // Skip the end_function element
                    if element.as_rule() == Rule::end_function {
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

            Ok(Some(Statement::Function { name, args, body }))
        },
        Rule::function_call => {
            let mut inner = pair.into_inner();
            let name = inner.next()
                .context("Missing function name in function call!")?
                .as_str().to_string();
            
            let mut args = Vec::new();
            // Collect all remaining arguments (expressions)
            while let Some(arg_pair) = inner.next() {
                if arg_pair.as_rule() == Rule::semicolon {
                    break;
                }
                
                let expr = build_expr(arg_pair)
                    .context("Failed to build expression in function call!")?;
                args.push(expr);
            }

            Ok(Some(Statement::FunctionCall { name, args }))
        },
        // Ignored
        Rule::begin | Rule::end | Rule::EOI | Rule::end_procedure | Rule::end_function => Ok(None),
        other => bail!("Unexpected statement: {:?}", other),
    }
}

fn build_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::function_call => {
                let mut inner = primary.into_inner();
                let name = inner.next()
                    .context("Missing function name in function call!")?
                    .as_str().to_string();
                
                let mut args = Vec::new();
                // Collect all remaining arguments (expressions)
                while let Some(arg_pair) = inner.next() {
                    if arg_pair.as_rule() == Rule::semicolon {
                        break;
                    }
                    
                    let expr = build_expr(arg_pair)
                        .context("Failed to build expression in function call!")?;
                    args.push(expr);
                }

                Ok(Expr::FunctionCall { name, args })
            },
            Rule::number => {
                let n = primary.as_str().parse::<i32>()?;
                Ok(Expr::Number(n))
            }
            Rule::string => {
                let s = primary.as_str();
                // Remove the surrounding quotes
                let s = &s[1..s.len()-1];
                Ok(Expr::String(s.to_string()))
            }
            Rule::identifier => Ok(Expr::Var(primary.as_str().to_string())),
            Rule::exp => {
                let mut inner = primary.into_inner();
                let expr_pair = inner.next()
                    .context("Missing inner expression for `EXP`!")?;
                let expr = Box::new(build_expr(expr_pair)?);
                Ok(Expr::Exp { expr })
            },
            Rule::cm => {
                let mut inner = primary.into_inner();
                let expr_pair = inner.next()
                    .context("Missing inner expression for `CM`!")?;
                let expr = Box::new(build_expr(expr_pair)?);
                Ok(Expr::Complex { expr })
            },
            Rule::expr => build_expr(primary),
            _ => bail!("Unexpected primary expr: {:?}", primary.as_rule()),
        })
        .map_infix(|
            left,
            op,
            right
        | match op.as_rule() {
            Rule::add => Ok(Expr::Add {
                left: Box::new(left?),
                right: Box::new(right?),
            }),
            Rule::concat => {
                let left = left?;
                let right = right?;

                let terms = if let Expr::Concat{ mut terms } = left {
                    terms.push(Box::new(right));
                    terms
                } else {
                    vec![Box::new(left), Box::new(right)]
                };

                Ok(Expr::Concat { terms })
            },
            _ => bail!("Unexpected infix operator: {:?}", op.as_rule()),
        })
        .parse(pair.into_inner())
}
