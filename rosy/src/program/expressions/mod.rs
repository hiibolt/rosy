pub mod var_expr;
pub mod concat;
pub mod complex_convert;
pub mod extract;
pub mod add;
pub mod sub;
pub mod mult;
pub mod div;
pub mod eq;
pub mod neq;
pub mod lt;
pub mod gt;
pub mod lte;
pub mod gte;
pub mod not;
pub mod string_convert;
pub mod logical_convert;
pub mod function_call;
pub mod da;
pub mod cd;
pub mod length;
pub mod sin;
pub mod sqr;
pub mod vmax;
pub mod lst;
pub mod lcm;
pub mod lcd;
pub mod derive;
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
use crate::program::expressions::cd::CDExpr;
use crate::program::expressions::length::LengthExpr;
use crate::program::expressions::sin::SinExpr;
use crate::program::expressions::sqr::SqrExpr;
use crate::program::expressions::vmax::VmaxExpr;
use crate::program::expressions::lst::LstExpr;
use crate::program::expressions::lcm::LcmExpr;
use crate::program::expressions::lcd::LcdExpr;
use crate::program::expressions::add::AddExpr;
use crate::program::expressions::sub::SubExpr;
use crate::program::expressions::mult::MultExpr;
use crate::program::expressions::div::DivExpr;
use crate::program::expressions::eq::EqExpr;
use crate::program::expressions::neq::NeqExpr;
use crate::program::expressions::lt::LtExpr;
use crate::program::expressions::gt::GtExpr;
use crate::program::expressions::lte::LteExpr;
use crate::program::expressions::gte::GteExpr;
use crate::program::expressions::not::NotExpr;
use crate::program::expressions::concat::ConcatExpr;
use crate::program::expressions::extract::ExtractExpr;
use crate::program::expressions::derive::DeriveExpr;
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
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    Not,
    Concat,
    Extract,
    Complex,
    StringConvert,
    LogicalConvert,
    DA,
    CD,
    Length,
    Sin,
    Sqr,
    Vmax,
    Lst,
    Lcm,
    Lcd,
    Derive,
    FunctionCall,
}

impl FromRule for Expr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Expr>> {
        let result = PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::not_expr => {
                    // NOT expression: parse the inner operand
                    // The inner can be: boolean, variable_identifier, or expr (parenthesized)
                    let mut inner = primary.into_inner();
                    let operand_pair = inner.next()
                        .ok_or_else(|| anyhow::anyhow!("NOT expression missing operand"))?;
                    
                    // Handle different operand types
                    let operand = match operand_pair.as_rule() {
                        Rule::boolean => {
                            let b = bool::from_rule(operand_pair)?
                                .ok_or_else(|| anyhow::anyhow!("Expected boolean"))?;
                            Expr {
                                enum_variant: ExprEnum::Boolean,
                                inner: Box::new(b),
                            }
                        },
                        Rule::variable_identifier => {
                            let var_expr = VarExpr::from_rule(operand_pair)?
                                .ok_or_else(|| anyhow::anyhow!("Expected VarExpr"))?;
                            Expr {
                                enum_variant: ExprEnum::Var,
                                inner: Box::new(var_expr),
                            }
                        },
                        Rule::expr => {
                            Expr::from_rule(operand_pair)?
                                .ok_or_else(|| anyhow::anyhow!("Failed to parse NOT operand expression"))?
                        },
                        other => {
                            return Err(anyhow::anyhow!("Unexpected NOT operand type: {:?}", other));
                        }
                    };
                    
                    Ok(Expr {
                        enum_variant: ExprEnum::Not,
                        inner: Box::new(NotExpr {
                            operand: Box::new(operand),
                        }),
                    })
                },
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
                Rule::cd_intrinsic => {
                    let cd_expr = CDExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::CD,
                        inner: Box::new(cd_expr.ok_or_else(|| anyhow::anyhow!("Expected CDExpr"))?),
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
                Rule::sqr => {
                    let sqr_expr = SqrExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Sqr,
                        inner: Box::new(sqr_expr.ok_or_else(|| anyhow::anyhow!("Expected SqrExpr"))?),
                    })
                },
                Rule::vmax => {
                    let vmax_expr = VmaxExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Vmax,
                        inner: Box::new(vmax_expr.ok_or_else(|| anyhow::anyhow!("Expected VmaxExpr"))?),
                    })
                },
                Rule::lst => {
                    let lst_expr = LstExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Lst,
                        inner: Box::new(lst_expr.ok_or_else(|| anyhow::anyhow!("Expected LstExpr"))?),
                    })
                },
                Rule::lcm => {
                    let lcm_expr = LcmExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Lcm,
                        inner: Box::new(lcm_expr.ok_or_else(|| anyhow::anyhow!("Expected LcmExpr"))?),
                    })
                },
                Rule::lcd => {
                    let lcd_expr = LcdExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Lcd,
                        inner: Box::new(lcd_expr.ok_or_else(|| anyhow::anyhow!("Expected LcdExpr"))?),
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
                Rule::derive => {
                    let left = left.context("...while transpiling object of `derive` (%) expression")?;
                    let right = right.context("...while transpiling index of `derive` (%) expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Derive,
                        inner: Box::new(DeriveExpr {
                            object: Box::new(left),
                            index: Box::new(right),
                        })
                    })
                },
                Rule::eq => {
                    let left = left.context("...while transpiling left-hand side of `eq` expression")?;
                    let right = right.context("...while transpiling right-hand side of `eq` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Eq,
                        inner: Box::new(EqExpr {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    })
                },
                Rule::neq => {
                    let left = left.context("...while transpiling left-hand side of `neq` expression")?;
                    let right = right.context("...while transpiling right-hand side of `neq` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Neq,
                        inner: Box::new(NeqExpr {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    })
                },
                Rule::lt => {
                    let left = left.context("...while transpiling left-hand side of `lt` expression")?;
                    let right = right.context("...while transpiling right-hand side of `lt` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Lt,
                        inner: Box::new(LtExpr {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    })
                },
                Rule::gt => {
                    let left = left.context("...while transpiling left-hand side of `gt` expression")?;
                    let right = right.context("...while transpiling right-hand side of `gt` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Gt,
                        inner: Box::new(GtExpr {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    })
                },
                Rule::lte => {
                    let left = left.context("...while transpiling left-hand side of `lte` expression")?;
                    let right = right.context("...while transpiling right-hand side of `lte` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Lte,
                        inner: Box::new(LteExpr {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    })
                },
                Rule::gte => {
                    let left = left.context("...while transpiling left-hand side of `gte` expression")?;
                    let right = right.context("...while transpiling right-hand side of `gte` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Gte,
                        inner: Box::new(GteExpr {
                            left: Box::new(left),
                            right: Box::new(right),
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
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile (
        &self, context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        self.inner.transpile(context)
            .map_err(|err_vec| {
                add_context_to_all(err_vec, format!("...while transpiling expression: {:?}", self))
            })
    }
}