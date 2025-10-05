mod statements;

use pest::pratt_parser::PrattParser;
use pest_derive::Parser;
use anyhow::{bail, ensure, Context, Result};
use rosy_lib::{RosyBaseType, RosyType};

#[derive(Parser)]
#[grammar = "../../rosy.pest"]
pub struct CosyParser;

// Create a static PrattParser for expressions
lazy_static::lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined from lowest to highest priority
        PrattParser::new()
            // Lowest precedence: concatenation (&)
            .op(Op::infix(concat, Left))
            // Medium precedence: extraction (|)
            .op(Op::infix(extract, Left))
            // Highest precedence: addition (+)
            .op(Op::infix(add, Left))
    };
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct VariableData {
    pub name: String,
    pub r#type: RosyType,
    pub dimension_exprs: Vec<Expr>
}
#[derive(Debug)]
pub struct VarDeclStatement {
    pub data: VariableData
}
#[derive(Debug)]
pub struct ProcedureStatement {
    pub name: String,
    pub args: Vec<VariableData>,
    pub body: Vec<Statement>
}
#[derive(Debug)]
pub struct FunctionStatement {
    pub name: String,
    pub args: Vec<VariableData>,
    pub return_type: RosyType,
    pub body: Vec<Statement>
}
#[derive(Debug, Clone, PartialEq)]
pub struct VariableIdentifier {
    pub name: String,
    pub indicies: Vec<Expr>
}
#[derive(Debug)]
pub struct AssignStatement {
    pub identifier: VariableIdentifier,
    pub value: Expr,
}
#[derive(Debug)]
pub struct WriteStatement {
    pub unit: u8,
    pub exprs: Vec<Expr>,
}
#[derive(Debug)]
pub struct FunctionCallStatement {
    pub name: String,
    pub args: Vec<Expr>,
}
#[derive(Debug)]
pub struct ProcedureCallStatement {
    pub name: String,
    pub args: Vec<Expr>,
}
#[derive(Debug)]
pub struct LoopStatement {
    pub iterator: String,
    pub start: Expr,
    pub end: Expr,
    pub step: Option<Expr>,
    pub body: Vec<Statement>,
}
#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expr,
    pub then_body: Vec<Statement>,
    pub elseif_clauses: Vec<ElseIfClause>,
    pub else_body: Option<Vec<Statement>>,
}
#[derive(Debug)]
pub struct ReadStatement {
    pub unit: u8,
    pub identifier: VariableIdentifier
}
#[derive(Debug)]
pub struct PLoopStatement {
    pub iterator: String,
    pub start: Expr,
    pub end: Expr,
    pub body: Vec<Statement>,
    pub commutivity_rule: Option<u8>,
    pub output: VariableIdentifier
}
#[derive(Debug)]
pub enum Statement {
    VarDecl(VarDeclStatement),
    Write(WriteStatement),
    Read(ReadStatement),
    Assign(AssignStatement),
    Procedure(ProcedureStatement),
    ProcedureCall(ProcedureCallStatement),
    Function(FunctionStatement),
    FunctionCall(FunctionCallStatement),
    Loop(LoopStatement),
    PLoop(PLoopStatement),
    If(IfStatement),
}

#[derive(Debug)]
pub struct ElseIfClause {
    pub condition: Expr,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConcatExpr {
    pub terms: Vec<Expr>
}
#[derive(Debug, Clone, PartialEq)]
pub struct ExtractExpr {
    pub object: Box<Expr>,
    pub index: Box<Expr>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexExpr {
    pub expr: Box<Expr>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct VarExpr {
    pub identifier: VariableIdentifier,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AddExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StringConvertExpr {
    pub expr: Box<Expr>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCallExpr {
    pub name: String,
    pub args: Vec<Expr>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Var(VarExpr),
    Add(AddExpr),
    Concat(ConcatExpr),
    Extract(ExtractExpr),
    Complex(ComplexExpr),
    StringConvert(StringConvertExpr),
    FunctionCall(FunctionCallExpr),
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

    let base_type: RosyBaseType = type_str
        .as_str()
        .try_into()
        .with_context(|| format!("Unknown type: {type_str}"))?;
    let r#type = RosyType::new(base_type, dimensions.len()); 

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
        Rule::ploop => statements::build_ploop(pair).context("...while building ploop statement!"),
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

fn build_variable_identifier(pair: pest::iterators::Pair<Rule>) -> Result<VariableIdentifier> {
    ensure!(pair.as_rule() == Rule::variable_identifier, 
        "Expected `variable_identifier` rule when building variable identifier, found: {:?}", pair.as_rule());
        
    let mut inner = pair.into_inner();
    let name = inner.next()
        .context("Missing variable name in indexed identifier!")?
        .as_str().to_string();
    
    let indicies = if let Some(next) = inner.next() {
        let mut indices = Vec::new();
        let mut inner_indices = next.into_inner();
        while let Some(index_pair) = inner_indices.next() {
            let expr = build_expr(index_pair)
                .context("Failed to build expression in indexed identifier!")?;
            indices.push(expr);
        }
        indices
    } else {
        Vec::new()
    };

    Ok(VariableIdentifier {
        name,
        indicies
    })
}

fn build_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::variable_identifier => Ok(Expr::Var(VarExpr {
                identifier: build_variable_identifier(primary)
                    .context("Failed to build variable identifier!")?
            })),
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

                Ok(Expr::FunctionCall(FunctionCallExpr { name, args }))
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
            },
            Rule::string => {
                let s = primary.as_str();
                // Remove the surrounding quotes
                let s = &s[1..s.len()-1];
                Ok(Expr::String(s.to_string()))
            },
            Rule::cm => {
                let mut inner = primary.into_inner();
                let expr_pair = inner.next()
                    .context("Missing inner expression for `CM`!")?;
                let expr = Box::new(build_expr(expr_pair)?);
                Ok(Expr::Complex(ComplexExpr { expr }))
            },
            Rule::st => {
                let mut inner = primary.into_inner();
                let expr_pair = inner.next()
                    .context("Missing inner expression for `ST`!")?;
                let expr = Box::new(build_expr(expr_pair)?);
                Ok(Expr::StringConvert(StringConvertExpr { expr }))
            },
            Rule::expr => build_expr(primary),
            _ => bail!("Unexpected primary expr: {:?}", primary.as_rule()),
        })
        .map_infix(|
            left,
            op,
            right
        | match op.as_rule() {
            Rule::add => Ok(Expr::Add(AddExpr {
                left: Box::new(left?),
                right: Box::new(right?),
            })),
            Rule::concat => {
                let left = left?;
                let right = right?;

                let terms = if let Expr::Concat(ConcatExpr { mut terms }) = left {
                    terms.push(right);
                    terms
                } else {
                    vec![left, right]
                };

                Ok(Expr::Concat(ConcatExpr { terms }))
            },
            Rule::extract => Ok(Expr::Extract(ExtractExpr {
                object: Box::new(left?),
                index: Box::new(right?),
            })),
            _ => bail!("Unexpected infix operator: {:?}", op.as_rule()),
        })
        .parse(pair.into_inner())
}
