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

use crate::{ast::{FromRule, PRATT_PARSER, Rule}, rosy_lib::RosyType, transpile::{TranspileWithType, TypeOf, add_context_to_all}};
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
                Rule::variable_identifier => {
                    let var_expr = VarExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Var,
                        inner: Box::new(var_expr.ok_or_else(|| anyhow::anyhow!("Expected VarExpr"))?),
                    })
                },
                Rule::function_call => {
                    let func_expr = FunctionCallExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::FunctionCall,
                        inner: Box::new(func_expr.ok_or_else(|| anyhow::anyhow!("Expected FunctionCallExpr"))?),
                    })
                },
                Rule::number => {
                    let n = f64::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Number,
                        inner: Box::new(n.ok_or_else(|| anyhow::anyhow!("Expected number"))?),
                    })
                }
                Rule::boolean => {
                    let b = bool::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Boolean,
                        inner: Box::new(b.ok_or_else(|| anyhow::anyhow!("Expected boolean"))?),
                    })
                },
                Rule::string => {
                    let s = String::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::String,
                        inner: Box::new(s.ok_or_else(|| anyhow::anyhow!("Expected string"))?),
                    })
                },
                Rule::cm => {
                    let cm_expr = ComplexConvertExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Complex,
                        inner: Box::new(cm_expr.ok_or_else(|| anyhow::anyhow!("Expected ComplexConvertExpr"))?),
                    })
                },
                Rule::st => {
                    let st_expr = StringConvertExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::StringConvert,
                        inner: Box::new(st_expr.ok_or_else(|| anyhow::anyhow!("Expected StringConvertExpr"))?),
                    })
                },
                Rule::lo => {
                    let lo_expr = LogicalConvertExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::LogicalConvert,
                        inner: Box::new(lo_expr.ok_or_else(|| anyhow::anyhow!("Expected LogicalConvertExpr"))?),
                    })
                },
                Rule::da => {
                    let da_expr = DAExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::DA,
                        inner: Box::new(da_expr.ok_or_else(|| anyhow::anyhow!("Expected DAExpr"))?),
                    })
                },
                Rule::length => {
                    let length_expr = LengthExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Length,
                        inner: Box::new(length_expr.ok_or_else(|| anyhow::anyhow!("Expected LengthExpr"))?),
                    })
                },
                Rule::sin => {
                    let sin_expr = SinExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Sin,
                        inner: Box::new(sin_expr.ok_or_else(|| anyhow::anyhow!("Expected SinExpr"))?),
                    })
                },
                Rule::expr => {
                    // handle parenthesized expressions by recursively parsing
                    Expr::from_rule(primary)
                        .context("Failed to build expression for parenthesized `expr`")?
                        .ok_or_else(|| anyhow::anyhow!("Expected expression for parenthesized `expr`"))
                },
                _ => bail!("Unexpected primary expr: {:?}", primary.as_rule()),
            })
            .map_infix(|
                left,
                op,
                right
            | match op.as_rule() {
                Rule::add => {
                    let left = left.context("...while transpiling left-hand side of `add` expression")?;
                    let right = right.context("...while transpiling right-hand side of `add` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Add,
                        inner: Box::new(AddExpr {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    })
                },
                Rule::sub => {
                    let left = left.context("...while transpiling left-hand side of `sub` expression")?;
                    let right = right.context("...while transpiling right-hand side of `sub` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Sub,
                        inner: Box::new(SubExpr {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    })
                },
                Rule::mult => {
                    let left = left.context("...while transpiling left-hand side of `mult` expression")?;
                    let right = right.context("...while transpiling right-hand side of `mult` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Mult,
                        inner: Box::new(MultExpr {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    })
                },
                Rule::div => {
                    let left = left.context("...while transpiling left-hand side of `div` expression")?;
                    let right = right.context("...while transpiling right-hand side of `div` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Div,
                        inner: Box::new(DivExpr {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    })
                },
                Rule::concat => {
                    let left = left.context("...while transpiling left-hand side of `concat` expression")?;
                    let right = right.context("...while transpiling right-hand side of `concat` expression")?;

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
                Rule::extract => {
                    let left = left.context("...while transpiling object of `extract` expression")?;
                    let right = right.context("...while transpiling index of `extract` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Extract,
                        inner: Box::new(ExtractExpr {
                            object: Box::new(left),
                            index: Box::new(right),
                        })
                    })
                },
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