pub mod var_expr;
pub mod concat;
pub mod complex_convert;
pub mod extract;
pub mod add;
pub mod sub;
pub mod mult;
pub mod div;
pub mod string_convert;
pub mod logical_convert;
pub mod function_call;
pub mod da;
pub mod length;
pub mod sin;
pub mod number;
pub mod string;
pub mod boolean;
pub mod variable_identifier;

use crate::{ast::{FromRule, PRATT_PARSER, Rule}, program::expressions::variable_identifier::VariableIdentifier, rosy_lib::RosyType, transpile::{TranspileWithType, TypeOf, add_context_to_all}};
use crate::transpile::{Transpile, TranspilationInputContext, TranspilationOutput};
use crate::program::expressions::var_expr::VarExpr;
use crate::program::expressions::function_call::FunctionCallExpr;
use crate::program::expressions::complex_convert::ComplexConvertExpr;
use crate::program::expressions::string_convert::StringConvertExpr;
use crate::program::expressions::logical_convert::LogicalConvertExpr;
use crate::program::expressions::da::DAExpr;
use crate::program::expressions::length::LengthExpr;
use crate::program::expressions::sin::SinExpr;
use crate::program::expressions::add::AddExpr;
use crate::program::expressions::sub::SubExpr;
use crate::program::expressions::mult::MultExpr;
use crate::program::expressions::div::DivExpr;
use crate::program::expressions::concat::ConcatExpr;
use crate::program::expressions::extract::ExtractExpr;
use anyhow::{Context, Error, Result, bail};

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

impl FromRule for Expr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Expr>> {
        let result = PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::variable_identifier => Ok(Expr {
                    enum_variant: ExprEnum::Var,
                    inner: Box::new(VarExpr {
                        identifier: VariableIdentifier::from_rule(primary)
                            .context("Failed to build variable identifier!")?
                            .ok_or_else(|| anyhow::anyhow!("Expected variable identifier"))?,
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
                            
                            let expr = Expr::from_rule(arg_pair)
                                .context("Failed to build expression in function call!")?
                                .ok_or_else(|| anyhow::anyhow!("Expected expression in function call"))?;
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
                    let expr = Box::new(Expr::from_rule(expr_pair)
                        .context("Failed to build expression for `CM`")?
                        .ok_or_else(|| anyhow::anyhow!("Expected expression for `CM`"))?);
                    Ok(Expr {
                        enum_variant: ExprEnum::Complex,
                        inner: Box::new(ComplexConvertExpr { expr })
                    })
                },
                Rule::st => {
                    let mut inner = primary.into_inner();
                    let expr_pair = inner.next()
                        .context("Missing inner expression for `ST`!")?;
                    let expr = Box::new(Expr::from_rule(expr_pair)
                        .context("Failed to build expression for `ST`")?
                        .ok_or_else(|| anyhow::anyhow!("Expected expression for `ST`"))?);
                    Ok(Expr {
                        enum_variant: ExprEnum::StringConvert,
                        inner: Box::new(StringConvertExpr { expr })
                    })
                },
                Rule::lo => {
                    let mut inner = primary.into_inner();
                    let expr_pair = inner.next()
                        .context("Missing inner expression for `LO`!")?;
                    let expr = Box::new(Expr::from_rule(expr_pair)
                        .context("Failed to build expression for `LO`")?
                        .ok_or_else(|| anyhow::anyhow!("Expected expression for `LO`"))?);
                    Ok(Expr {
                        enum_variant: ExprEnum::LogicalConvert,
                        inner: Box::new(LogicalConvertExpr { expr })
                    })
                },
                Rule::da => {
                    let mut inner = primary.into_inner();
                    let expr_pair = inner.next()
                        .context("Missing inner expression for `DA`!")?;
                    let index = Box::new(Expr::from_rule(expr_pair)
                        .context("Failed to build expression for `DA`")?
                        .ok_or_else(|| anyhow::anyhow!("Expected expression for `DA`"))?);
                    Ok(Expr {
                        enum_variant: ExprEnum::DA,
                        inner: Box::new(DAExpr { index })
                    })
                },
                Rule::length => {
                    let mut inner = primary.into_inner();
                    let expr_pair = inner.next()
                        .context("Missing inner expression for `LENGTH`!")?;
                    let expr = Box::new(Expr::from_rule(expr_pair)
                        .context("Failed to build expression for `LENGTH`")?
                        .ok_or_else(|| anyhow::anyhow!("Expected expression for `LENGTH`"))?);
                    Ok(Expr {
                        enum_variant: ExprEnum::Length,
                        inner: Box::new(LengthExpr { expr })
                    })
                },
                Rule::sin => {
                    let mut inner = primary.into_inner();
                    let expr_pair = inner.next()
                        .context("Missing inner expression for `SIN`!")?;
                    let expr = Box::new(Expr::from_rule(expr_pair)
                        .context("Failed to build expression for `SIN`")?
                        .ok_or_else(|| anyhow::anyhow!("Expected expression for `SIN`"))?);
                    Ok(Expr {
                        enum_variant: ExprEnum::Sin,
                        inner: Box::new(SinExpr { expr })
                    })
                },
                /*
                Rule::expr => Ok(Expr::from_rule(primary)
                    .context("Failed to build expression for `expr`")?
                    .ok_or_else(|| anyhow::anyhow!("Expected expression for `expr`"))?), */
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
            .parse(pair.into_inner());

        result.map(Some)
    }
}
impl TypeOf for Expr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        self.inner.type_of(context)
    }
}
impl Transpile for Expr {
    fn transpile (
        &self, context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        self.inner.transpile(context)
            .map_err(|err_vec| {
                add_context_to_all(err_vec, format!("...while transpiling expression: {:?}", self))
            })
    }
}