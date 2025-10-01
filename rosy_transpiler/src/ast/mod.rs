mod statements;

use crate::parsing::{Rule, PRATT_PARSER};
use anyhow::{bail, ensure, Context, Result};
use rosy_lib::RosyType;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableData {
    pub name: String,
    pub r#type: RosyType,
    pub dimensions: Vec<Expr>
}
#[derive(Debug)]
pub enum Statement {
    VarDecl { data: VariableData },
    Write { unit: u8, exprs: Vec<Expr> },
    Read { unit: u8, name: String },
    Assign { name: String, value: Expr, indicies: Vec<Expr> },
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

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Var(String),
    VarIndexing { name: String, indices: Vec<Expr> },
    Add { left: Box<Expr>, right: Box<Expr> },
    Concat { terms: Vec<Expr> },
    Extract { object: Box<Expr>, index: Box<Expr> },
    Exp { expr: Box<Expr> },
    Complex { expr: Box<Expr> },
    StringConvert { expr: Box<Expr> },
    FunctionCall { name: String, args: Vec<Expr> },
}

fn build_type (pair: pest::iterators::Pair<Rule>) -> Result<(RosyType, Vec<Expr>)> {
    ensure!(pair.as_rule() == Rule::r#type, 
        "Expected `type` rule when building type, found: {:?}", pair.as_rule());
        
    let mut inner_pair = pair.into_inner();
    let type_str = inner_pair.next()
        .context("Missing type string when building var decl!")?
        .as_str().to_string();
    let mut dimensions = Vec::new();
    while let Some(dim_pair) = inner_pair.next() {
        let expr = build_expr(dim_pair)
            .context("Failed to build dimension expression in variable declaration!")?;
        dimensions.push(expr);
    }

    let r#type: RosyType = type_str.as_str()
        .try_into()
        .with_context(|| format!("Unknown type: {type_str}"))?;

    Ok((r#type, dimensions))
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
        Rule::var_decl => statements::build_var_decl(pair).context("...while building variable declaration!"),
        Rule::write => statements::build_write(pair).context("...while building write statement!"),
        Rule::read => statements::build_read(pair).context("...while building read statement!"),
        Rule::assignment => statements::build_assignment(pair).context("...while building assignment statement!"),
        Rule::r#loop => statements::build_loop(pair).context("...while building loop statement!"),
        Rule::procedure => statements::build_procedure(pair).context("...while building procedure declaration!"),
        Rule::procedure_call => statements::build_procedure_call(pair).context("...while building procedure call!"),
        Rule::function => statements::build_function(pair).context("...while building function declaration!"),
        Rule::function_call => statements::build_function_call(pair).context("...while building function call!"),
        Rule::if_statement => statements::build_if(pair).context("...while building if statement!"),

        // Ignored
        Rule::begin | Rule::end | Rule::EOI | Rule::end_procedure | 
        Rule::end_function | Rule::end_loop | Rule::endif => Ok(None),
        other => bail!("Unexpected statement: {:?}", other),
    }
}

fn build_indexed_identifier(pair: pest::iterators::Pair<Rule>) -> Result<(String, Vec<Expr>)> {
    ensure!(pair.as_rule() == Rule::indexed_identifier, 
        "Expected `indexed_identifier` rule when building indexed identifier, found: {:?}", pair.as_rule());
        
    let mut inner = pair.into_inner();
    let name = inner.next()
        .context("Missing variable name in indexed identifier!")?
        .as_str().to_string();
    
    let indices = {
        let mut indices = Vec::new();
        while let Some(index_pair) = inner.next() {
            let expr = build_expr(index_pair)
                .context("Failed to build expression in indexed identifier!")?;
            indices.push(expr);
        }
        indices
    };

    Ok((name, indices))
}

fn build_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::indexed_identifier => {
                let mut inner = primary.into_inner();
                let name = inner.next()
                    .context("Missing variable name in indexed identifier!")?
                    .as_str().to_string();
                
                let indices = {
                    let mut indices = Vec::new();
                    while let Some(index_pair) = inner.next() {
                        let expr = build_expr(index_pair)
                            .context("Failed to build expression in indexed identifier!")?;
                        indices.push(expr);
                    }
                    indices
                };

                Ok(Expr::VarIndexing { name, indices })
            },
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
                let n = primary.as_str().parse::<f64>()?;
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
            Rule::st => {
                let mut inner = primary.into_inner();
                let expr_pair = inner.next()
                    .context("Missing inner expression for `ST`!")?;
                let expr = Box::new(build_expr(expr_pair)?);
                Ok(Expr::StringConvert { expr })
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
                    terms.push(right);
                    terms
                } else {
                    vec![left, right]
                };

                Ok(Expr::Concat { terms })
            },
            Rule::extract => Ok(Expr::Extract {
                object: Box::new(left?),
                index: Box::new(right?),
            }),
            _ => bail!("Unexpected infix operator: {:?}", op.as_rule()),
        })
        .parse(pair.into_inner())
}
