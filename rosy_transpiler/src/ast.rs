use crate::parsing::{Rule, PRATT_PARSER};
use anyhow::{bail, ensure, Context, Result};
use rosy_lib::RosyType;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableData {
    pub name: String,
    pub r#type: RosyType
}
#[derive(Debug)]
pub enum Statement {
    VarDecl { data: VariableData },
    Write { unit: u8, exprs: Vec<Expr> },
    Read { unit: u8, name: String },
    Assign { name: String, value: Expr },
    Procedure { name: String, args: Vec<VariableData>, body: Vec<Statement> },
    ProcedureCall { name: String, args: Vec<Expr> },
    Function { name: String, args: Vec<VariableData>, return_type: RosyType, body: Vec<Statement> },
    FunctionCall { name: String, args: Vec<Expr> },
    Loop { iterator: String, start: Expr, end: Expr, step: Option<Expr>, body: Vec<Statement> },
    If { condition: Expr, then_body: Vec<Statement>, elseif_clauses: Vec<ElseIfClause>, else_body: Option<Vec<Statement>> },
}

#[derive(Debug)]
pub struct ElseIfClause {
    pub condition: Expr,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    String(String),
    Boolean(bool),
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
            let type_str = inner.next()
                .context("Missing first token `type` when building var decl!")?
                .as_str().to_string();
            let name = inner.next()
                .context("Missing second token `variable_name`!")?
                .as_str().to_string();

            let variable_data = VariableData {
                name,
                r#type: type_str.as_str().try_into()
                    .with_context(|| format!("Unknown type: {type_str}"))?
            };

            Ok(Some(Statement::VarDecl {
                data: variable_data
            }))
        },
        Rule::write => {
            let mut inner = pair.into_inner();

            let unit = inner.next()
                .context("Missing first token `unit`!")?
                .as_str()
                .parse::<u8>()
                .context("Failed to parse `unit` as u8 in `write` statement!")?;

            let exprs = {
                let mut exprs = Vec::new();
                while let Some(expr_pair) = inner.next() {
                    if expr_pair.as_rule() == Rule::semicolon {
                        break;
                    }

                    let expr = build_expr(expr_pair)
                        .context("Failed to build expression in `write` statement!")?;
                    exprs.push(expr);
                }
                exprs
            };

            Ok(Some(Statement::Write { unit, exprs }))
        },
        Rule::read => {
            let mut inner = pair.into_inner();

            let unit = inner.next()
                .context("Missing first token `unit`!")?
                .as_str()
                .parse::<u8>()
                .context("Failed to parse `unit` as u8 in `read` statement!")?;

            let name = inner.next()
                .context("Missing second token `variable_name`!")?
                .as_str()
                .to_string();

            Ok(Some(Statement::Read { unit, name }))
        },
        Rule::assignment => {
            let mut inner = pair.into_inner();
            let name = inner.next()
                .context("Missing first token `variable_name`!")?
                .as_str().to_string();
            let expr_pair = inner.next()
                .context("Missing second token `expr`!")?;
            let expr = build_expr(expr_pair)?;
            Ok(Some(Statement::Assign { name, value: expr }))
        },
        Rule::r#loop => {
            let mut inner = pair.into_inner();
            let (iterator, start, end, step) = {
                let mut start_loop_inner = inner
                    .next()
                    .context("Missing first token `start_loop`!")?
                    .into_inner();

                let iterator = start_loop_inner.next()
                    .context("Missing first token `variable_name`!")?
                    .as_str().to_string();
                let start_pair = start_loop_inner.next()
                    .context("Missing second token `start_expr`!")?;
                let start = build_expr(start_pair)
                    .context("Failed to build `start` expression in `loop` statement!")?;
                let end_pair = start_loop_inner.next()
                    .context("Missing third token `end_expr`!")?;
                let end = build_expr(end_pair)
                    .context("Failed to build `end` expression in `loop` statement!")?;
                
                // Optional step expression
                let step = if let Some(step_pair) = start_loop_inner.next() {
                    if step_pair.as_rule() == Rule::expr {
                        Some(build_expr(step_pair)
                            .context("Failed to build `step` expression in `loop` statement!")?)
                    } else {
                        None
                    }
                } else {
                    None
                };

                (iterator, start, end, step)
            };

            let mut body = Vec::new();
            // Process remaining elements (statements and end)
            while let Some(element) = inner.next() {
                // Skip the end element
                if element.as_rule() == Rule::end {
                    break;
                }

                let pair_input = element.as_str();
                if let Some(stmt) = build_statement(element)
                    .with_context(|| format!("Failed to build statement from:\n{}", pair_input))? {
                    body.push(stmt);
                }
            }

            Ok(Some(Statement::Loop { iterator, start, end, step, body }))  
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
                // Collect all remaining argument names and types
                while let Some(arg_pair) = start_procedure_inner.next() {
                    if arg_pair.as_rule() == Rule::semicolon {
                        break;
                    }

                    ensure!(arg_pair.as_rule() == Rule::function_argument_name, 
                        "Expected function argument name, found: {:?}", arg_pair.as_rule());
                    let name = arg_pair.as_str();

                    let next_arg_pair = start_procedure_inner.next()
                        .context(format!("Missing type for function argument: {}", name))?;
                    ensure!(next_arg_pair.as_rule() == Rule::r#type, 
                        "Expected type for function argument, found: {:?}", next_arg_pair.as_rule());
                    let type_str = next_arg_pair.as_str();

                    let variable_data = VariableData {
                        name: name.to_string(),
                        r#type: type_str.try_into()
                            .with_context(|| format!("Unknown type: {type_str}"))?
                    };
                    args.push(variable_data);
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
            let (return_type, name, args) = {
                let mut start_function_inner = inner
                    .next()
                    .context("Missing first token `start_function`!")?
                    .into_inner();

                let type_str = start_function_inner.next()
                    .context("Missing return type in function declaration!")?
                    .as_str().to_string();
                let return_type = type_str.as_str().try_into()
                    .with_context(|| format!("Unknown type: {type_str}"))?;

                let name = start_function_inner.next()
                    .context("Missing function name!")?
                    .as_str().to_string();

                let mut args = Vec::new();
                // Collect all remaining argument names and types
                while let Some(arg_pair) = start_function_inner.next() {
                    if arg_pair.as_rule() == Rule::semicolon {
                        break;
                    }

                    ensure!(arg_pair.as_rule() == Rule::function_argument_name, 
                        "Expected function argument name, found: {:?}", arg_pair.as_rule());
                    let name = arg_pair.as_str();

                    let next_arg_pair = start_function_inner.next()
                        .context(format!("Missing type for function argument: {}", name))?;
                    ensure!(next_arg_pair.as_rule() == Rule::r#type, 
                        "Expected type for function argument, found: {:?}", next_arg_pair.as_rule());
                    let type_str = next_arg_pair.as_str();

                    let variable_data = VariableData {
                        name: name.to_string(),
                        r#type: type_str.try_into()
                            .with_context(|| format!("Unknown type: {type_str}"))?
                    };
                    args.push(variable_data);
                }

                (return_type, name, args)
            };

            let body = {
                let mut statements = vec!(
                    Statement::VarDecl { data: VariableData {
                        name: name.clone(),
                        r#type: RosyType::RE
                    } }
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

            Ok(Some(Statement::Function { name, args, return_type, body }))
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
        Rule::if_statement => {
            let mut inner = pair.into_inner();
            
            // Parse the main IF clause
            let (condition, then_body) = {
                let mut if_clause_inner = inner
                    .next()
                    .context("Missing if_clause!")?
                    .into_inner();
                
                let condition = build_expr(if_clause_inner.next()
                    .context("Missing condition in IF clause!")?)
                    .context("Failed to build IF condition expression!")?;
                
                let mut then_body = Vec::new();
                while let Some(stmt_pair) = if_clause_inner.next() {
                    if stmt_pair.as_rule() == Rule::semicolon {
                        continue;
                    }
                    
                    let pair_input = stmt_pair.as_str();
                    if let Some(stmt) = build_statement(stmt_pair)
                        .with_context(|| format!("Failed to build statement in IF body from:\n{}", pair_input))? {
                        then_body.push(stmt);
                    }
                }
                
                (condition, then_body)
            };
            
            // Parse ELSEIF clauses
            let mut elseif_clauses = Vec::new();
            let mut else_body = None;
            while let Some(element) = inner.next() {
                match element.as_rule() {
                    Rule::elseif_clause => {
                        let mut elseif_inner = element.into_inner();
                        
                        let condition = build_expr(elseif_inner.next()
                            .context("Missing condition in ELSEIF clause!")?)
                            .context("Failed to build ELSEIF condition expression!")?;
                        
                        let mut body = Vec::new();
                        while let Some(stmt_pair) = elseif_inner.next() {
                            if stmt_pair.as_rule() == Rule::semicolon {
                                continue;
                            }
                            
                            let pair_input = stmt_pair.as_str();
                            if let Some(stmt) = build_statement(stmt_pair)
                                .with_context(|| format!("Failed to build statement in ELSEIF body from:\n{}", pair_input))? {
                                body.push(stmt);
                            }
                        }
                        
                        elseif_clauses.push(ElseIfClause { condition, body });
                    },
                    Rule::else_clause => {
                        let mut else_inner = element.into_inner();
                        let mut body = Vec::new();
                        while let Some(stmt_pair) = else_inner.next() {
                            if stmt_pair.as_rule() == Rule::semicolon {
                                continue;
                            }
                            
                            let pair_input = stmt_pair.as_str();
                            if let Some(stmt) = build_statement(stmt_pair)
                                .with_context(|| format!("Failed to build statement in ELSE body from:\n{}", pair_input))? {
                                body.push(stmt);
                            }
                        }
                        else_body = Some(body);
                    },
                    Rule::endif => {
                        // End of IF statement
                        break;
                    },
                    _ => {
                        bail!("Unexpected element in IF statement: {:?}", element.as_rule());
                    }
                }
            }
            
            Ok(Some(Statement::If { condition, then_body, elseif_clauses, else_body }))
        },
        // Ignored
        Rule::begin | Rule::end | Rule::EOI | Rule::end_procedure | 
        Rule::end_function | Rule::end_loop | Rule::endif => Ok(None),
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
                
                let args = {
                    let mut args = Vec::new();
                    while let Some(arg_pair) = inner.next() {
                        if arg_pair.as_rule() == Rule::semicolon {
                            break;
                        }
                        
                        let expr = build_expr(arg_pair)
                            .context("Failed to build expression in function call!")?;
                        args.push(expr);
                    }
                    args
                };

                Ok(Expr::FunctionCall { name, args })
            },
            Rule::number => {
                
                let n = primary.as_str().parse::<i32>()?;
                Ok(Expr::Number(n))
            }
            Rule::boolean => {
                let b = match primary.as_str() {
                    "TRUE" => true,
                    "FALSE" => false,
                    _ => bail!("Unexpected boolean value: {}", primary.as_str()),
                };
                Ok(Expr::Boolean(b))
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
