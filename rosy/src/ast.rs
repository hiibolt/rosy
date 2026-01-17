use pest::pratt_parser::PrattParser;
use pest_derive::Parser;
use anyhow::{bail, ensure, Context, Result};
use crate::{rosy_lib::{RosyBaseType, RosyType}, transpile::{Transpile, TranspileWithType}};

#[derive(Parser)]
#[grammar = "../assets/rosy.pest"]
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
            // Extraction (|)
            .op(Op::infix(extract, Left))
            // Addition and Subtraction (same precedence)
            .op(Op::infix(add, Left) | Op::infix(sub, Left))
            // Multiplication and Division (same precedence)
            .op(Op::infix(mult, Left) | Op::infix(div, Left))
    };
}

pub trait StatementFromRule {
    fn from_rule ( pair: pest::iterators::Pair<Rule> ) -> Result<Option<Statement>>;
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct VariableDeclarationData {
    pub name: String,
    pub r#type: RosyType,
    pub dimension_exprs: Vec<Expr>
}
#[derive(Debug)]
pub struct VarDeclStatement {
    pub data: VariableDeclarationData
}
#[derive(Debug)]
pub struct ProcedureStatement {
    pub name: String,
    pub args: Vec<VariableDeclarationData>,
    pub body: Vec<Statement>
}
#[derive(Debug)]
pub struct FunctionStatement {
    pub name: String,
    pub args: Vec<VariableDeclarationData>,
    pub return_type: RosyType,
    pub body: Vec<Statement>
}
#[derive(Debug, PartialEq)]
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
pub struct DAInitStatement {
    pub order: Expr,
    pub number_of_variables: Expr,
}
#[derive(Debug)]
pub struct Statement {
    pub enum_variant: StatementEnum,
    pub inner: Box<dyn Transpile>,
}
#[derive(Debug)]
pub enum StatementEnum {
    DAInit,
    VarDecl,
    Write,
    Read,
    Assign,
    Procedure,
    ProcedureCall,
    Function,
    FunctionCall,
    Loop,
    PLoop,
    If,
}

#[derive(Debug)]
pub struct ElseIfClause {
    pub condition: Expr,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct ConcatExpr {
    pub terms: Vec<Expr>
}
#[derive(Debug, PartialEq)]
pub struct ExtractExpr {
    pub object: Box<Expr>,
    pub index: Box<Expr>,
}
#[derive(Debug, PartialEq)]
pub struct ComplexExpr {
    pub expr: Box<Expr>,
}
#[derive(Debug, PartialEq)]
pub struct VarExpr {
    pub identifier: VariableIdentifier,
}
#[derive(Debug, PartialEq)]
pub struct AddExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
#[derive(Debug, PartialEq)]
pub struct SubExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
#[derive(Debug, PartialEq)]
pub struct MultExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
#[derive(Debug, PartialEq)]
pub struct DivExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
#[derive(Debug, PartialEq)]
pub struct StringConvertExpr {
    pub expr: Box<Expr>,
}
#[derive(Debug, PartialEq)]
pub struct LogicalConvertExpr {
    pub expr: Box<Expr>,
}
#[derive(Debug, PartialEq)]
pub struct DAExpr {
    pub index: Box<Expr>,
}
#[derive(Debug, PartialEq)]
pub struct LengthExpr {
    pub expr: Box<Expr>,
}
#[derive(Debug, PartialEq)]
pub struct SinExpr {
    pub expr: Box<Expr>,
}
#[derive(Debug, PartialEq)]
pub struct FunctionCallExpr {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug)]
pub struct Expr {
    pub enum_variant: ExprEnum,
    pub inner: Box<dyn TranspileWithType>,
}
impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.enum_variant == other.enum_variant
    }
}
#[derive(Debug, PartialEq)]
pub enum ExprEnum {
    Number,
    String,
    Boolean,
    Var,
    Add,
    Sub,
    Mult,
    Div,
    Concat,
    Extract,
    Complex,
    StringConvert,
    LogicalConvert,
    DA,
    Length,
    Sin,
    FunctionCall,
}

pub fn build_type (pair: pest::iterators::Pair<Rule>) -> Result<(RosyType, Vec<Expr>)> {
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

pub fn build_statement (
    pair: pest::iterators::Pair<Rule>
) -> Result<Option<Statement>> {
    match pair.as_rule() {
        Rule::daini => DAInitStatement::from_rule(pair).context("...while building DA initialization statement!"),
        Rule::var_decl => VarDeclStatement::from_rule(pair).context("...while building variable declaration!"),
        Rule::write => WriteStatement::from_rule(pair).context("...while building write statement!"),
        Rule::read => ReadStatement::from_rule(pair).context("...while building read statement!"),
        Rule::assignment => AssignStatement::from_rule(pair).context("...while building assignment statement!"),
        Rule::r#loop => LoopStatement::from_rule(pair).context("...while building loop statement!"),
        Rule::ploop => PLoopStatement::from_rule(pair).context("...while building ploop statement!"),
        Rule::procedure => ProcedureStatement::from_rule(pair).context("...while building procedure declaration!"),
        Rule::procedure_call => ProcedureCallStatement::from_rule(pair).context("...while building procedure call!"),
        Rule::function => FunctionStatement::from_rule(pair).context("...while building function declaration!"),
        Rule::function_call => FunctionCallStatement::from_rule(pair).context("...while building function call!"),
        Rule::if_statement => IfStatement::from_rule(pair).context("...while building if statement!"),

        // Ignored
        Rule::begin | Rule::end | Rule::EOI | Rule::end_procedure | 
        Rule::end_function | Rule::end_loop | Rule::endif => Ok(None),
        other => bail!("Unexpected statement: {:?}", other),
    }
}

pub fn build_variable_identifier(pair: pest::iterators::Pair<Rule>) -> Result<VariableIdentifier> {
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

pub fn build_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::variable_identifier => Ok(Expr {
                enum_variant: ExprEnum::Var,
                inner: Box::new(VarExpr {
                    identifier: build_variable_identifier(primary)
                        .context("Failed to build variable identifier!")?
                })
            }),
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

                Ok(Expr {
                    enum_variant: ExprEnum::FunctionCall,
                    inner: Box::new(FunctionCallExpr { name, args })
                })
            },
            Rule::number => {
                let n = primary.as_str().parse::<f64>()?;
                Ok(Expr {
                    enum_variant: ExprEnum::Number,
                    inner: Box::new(n)
                })
            }
            Rule::boolean => {
                let b = match primary.as_str() {
                    "TRUE" => true,
                    "FALSE" => false,
                    _ => bail!("Unexpected boolean value: {}", primary.as_str()),
                };
                Ok(Expr {
                    enum_variant: ExprEnum::Boolean,
                    inner: Box::new(b)
                })
            },
            Rule::string => {
                let s = primary.as_str();
                // Remove the surrounding quotes
                let s = &s[1..s.len()-1];
                Ok(Expr {
                    enum_variant: ExprEnum::String,
                    inner: Box::new(s.to_string())
                })
            },
            Rule::cm => {
                let mut inner = primary.into_inner();
                let expr_pair = inner.next()
                    .context("Missing inner expression for `CM`!")?;
                let expr = Box::new(build_expr(expr_pair)?);
                Ok(Expr {
                    enum_variant: ExprEnum::Complex,
                    inner: Box::new(ComplexExpr { expr })
                })
            },
            Rule::st => {
                let mut inner = primary.into_inner();
                let expr_pair = inner.next()
                    .context("Missing inner expression for `ST`!")?;
                let expr = Box::new(build_expr(expr_pair)?);
                Ok(Expr {
                    enum_variant: ExprEnum::StringConvert,
                    inner: Box::new(StringConvertExpr { expr })
                })
            },
            Rule::lo => {
                let mut inner = primary.into_inner();
                let expr_pair = inner.next()
                    .context("Missing inner expression for `LO`!")?;
                let expr = Box::new(build_expr(expr_pair)?);
                Ok(Expr {
                    enum_variant: ExprEnum::LogicalConvert,
                    inner: Box::new(LogicalConvertExpr { expr })
                })
            },
            Rule::da => {
                let mut inner = primary.into_inner();
                let expr_pair = inner.next()
                    .context("Missing inner expression for `DA`!")?;
                let index = Box::new(build_expr(expr_pair)?);
                Ok(Expr {
                    enum_variant: ExprEnum::DA,
                    inner: Box::new(DAExpr { index })
                })
            },
            Rule::length => {
                let mut inner = primary.into_inner();
                let expr_pair = inner.next()
                    .context("Missing inner expression for `LENGTH`!")?;
                let expr = Box::new(build_expr(expr_pair)?);
                Ok(Expr {
                    enum_variant: ExprEnum::Length,
                    inner: Box::new(LengthExpr { expr })
                })
            },
            Rule::sin => {
                let mut inner = primary.into_inner();
                let expr_pair = inner.next()
                    .context("Missing inner expression for `SIN`!")?;
                let expr = Box::new(build_expr(expr_pair)?);
                Ok(Expr {
                    enum_variant: ExprEnum::Sin,
                    inner: Box::new(SinExpr { expr })
                })
            },
            Rule::expr => build_expr(primary),
            _ => bail!("Unexpected primary expr: {:?}", primary.as_rule()),
        })
        .map_infix(|
            left,
            op,
            right
        | match op.as_rule() {
            Rule::add => Ok(Expr {
                enum_variant: ExprEnum::Add,
                inner: Box::new(AddExpr {
                    left: Box::new(left?),
                    right: Box::new(right?),
                })
            }),
            Rule::sub => Ok(Expr {
                enum_variant: ExprEnum::Sub,
                inner: Box::new(SubExpr {
                    left: Box::new(left?),
                    right: Box::new(right?),
                })
            }),
            Rule::mult => Ok(Expr {
                enum_variant: ExprEnum::Mult,
                inner: Box::new(MultExpr {
                    left: Box::new(left?),
                    right: Box::new(right?),
                })
            }),
            Rule::div => Ok(Expr {
                enum_variant: ExprEnum::Div,
                inner: Box::new(DivExpr {
                    left: Box::new(left?),
                    right: Box::new(right?),
                })
            }),
            Rule::concat => {
                let left = left?;
                let right = right?;

                // If left is already a Concat, extend its terms instead of nesting
                if left.enum_variant == ExprEnum::Concat {
                    // Downcast through Any trait to take ownership of the ConcatExpr
                    let left_any: Box<dyn std::any::Any> = left.inner;
                    if let Ok(concat_expr) = left_any.downcast::<ConcatExpr>() {
                        let mut terms = concat_expr.terms;
                        terms.push(right);
                        Ok(Expr {
                            enum_variant: ExprEnum::Concat,
                            inner: Box::new(ConcatExpr { terms })
                        })
                    } else {
                        bail!("Failed to downcast Concat expression - internal inconsistency")
                    }
                } else {
                    let terms = vec![left, right];
                    Ok(Expr {
                        enum_variant: ExprEnum::Concat,
                        inner: Box::new(ConcatExpr { terms })
                    })
                }
            },
            Rule::extract => Ok(Expr {
                enum_variant: ExprEnum::Extract,
                inner: Box::new(ExtractExpr {
                    object: Box::new(left?),
                    index: Box::new(right?),
                })
            }),
            _ => bail!("Unexpected infix operator: {:?}", op.as_rule()),
        })
        .parse(pair.into_inner())
}
