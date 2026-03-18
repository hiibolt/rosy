//! # Expressions
//!
//! All expression types in the ROSY language. Expressions produce values and
//! have types determined at transpilation time via the [`TranspileableExpr`] trait.
//!
//! ## Sub-modules
//!
//! - **[`operators`]** — Binary and unary operators (`+`, `-`, `*`, `/`, `&`, `|`, `%`, comparisons, `NOT`, negation)
//! - **[`functions`]** — Built-in functions (math, conversion, system)
//! - **[`types`]** — Literal values (numbers, strings, booleans, `DA()`, `CD()`)
//! - **[`core`]** — Variable references and function call disambiguation
//!
//! ## Pratt Parser
//!
//! Binary operators are parsed using a Pratt parser with precedence levels
//! defined in the [`PRATT_PARSER`]. The parser produces an [`Expr`] AST node
//! wrapping a concrete expression type (e.g., [`AddExpr`]).
//!
//! ## Example
//!
//! ```text
//! {ROSY expression examples}
//! x + y * 2           {arithmetic with precedence}
//! 1 & 2 & 3           {vector concatenation}
//! vec|3               {extract 3rd element}
//! SIN(x)              {intrinsic function}
//! ST(42)              {type conversion}
//! DA(1)               {DA variable constructor}
//! ```

pub mod core;
pub mod functions;
pub mod operators;
pub mod types;

use std::collections::HashSet;
use crate::{ast::{FromRule, PRATT_PARSER, Rule}, resolve::{ExprRecipe, ScopeContext, TypeResolver, TypeSlot}, rosy_lib::RosyType, transpile::{TranspileableExpr, add_context_to_all}};
use crate::transpile::{Transpile, TranspilationInputContext, TranspilationOutput};

use crate::program::expressions::core::var_expr::VarExpr;

use crate::program::expressions::functions::conversion::complex_convert::ComplexConvertExpr;
use crate::program::expressions::functions::conversion::string_convert::StringConvertExpr;
use crate::program::expressions::functions::conversion::logical_convert::LogicalConvertExpr;
use crate::program::expressions::functions::math::trig::tan::TanExpr;
use crate::program::expressions::functions::math::trig::sin::SinExpr;
use crate::program::expressions::functions::math::sqr::SqrExpr;
use crate::program::expressions::functions::math::sqrt::SqrtExpr;
use crate::program::expressions::functions::math::exp::ExpExpr;
use crate::program::expressions::functions::math::vmax::VmaxExpr;
use crate::program::expressions::functions::math::lst::LstExpr;
use crate::program::expressions::functions::math::lcm::LcmExpr;
use crate::program::expressions::functions::math::lcd::LcdExpr;
use crate::program::expressions::functions::math::pow::PowExpr;

use crate::program::expressions::operators::add::AddExpr;
use crate::program::expressions::operators::sub::SubExpr;
use crate::program::expressions::operators::mult::MultExpr;
use crate::program::expressions::operators::div::DivExpr;
use crate::program::expressions::operators::eq::EqExpr;
use crate::program::expressions::operators::neq::NeqExpr;
use crate::program::expressions::operators::lt::LtExpr;
use crate::program::expressions::operators::gt::GtExpr;
use crate::program::expressions::operators::lte::LteExpr;
use crate::program::expressions::operators::gte::GteExpr;
use crate::program::expressions::operators::not::NotExpr;
use crate::program::expressions::operators::neg::NegExpr;
use crate::program::expressions::operators::concat::ConcatExpr;
use crate::program::expressions::operators::extract::ExtractExpr;
use crate::program::expressions::operators::derive::DeriveExpr;

use crate::program::expressions::functions::sys::length::LengthExpr;

use crate::program::expressions::types::da::DAExpr;
use crate::program::expressions::types::cd::CDExpr;
use anyhow::{Context, Error, Result, bail};

#[derive(Debug)]
pub struct Expr {
    pub enum_variant: ExprEnum,
    pub inner: Box<dyn TranspileableExpr>,
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
    Pow,
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
    Sqrt,
    Exp,
    Tan,
    Vmax,
    Lst,
    Lcm,
    Lcd,
    Neg,
    Derive,
}

impl FromRule for Expr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Expr>> {
        let result = PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::neg_expr => {
                    let mut inner = primary.into_inner();
                    let operand_pair = inner.next()
                        .ok_or_else(|| anyhow::anyhow!("Negation expression missing operand"))?;
                    let operand = Expr::from_rule(operand_pair)
                        .context("Failed to parse negation operand")?
                        .ok_or_else(|| anyhow::anyhow!("Expected expression in negation"))?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Neg,
                        inner: Box::new(NegExpr { operand: Box::new(operand) }),
                    })
                },
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
                Rule::sqrt_fn => {
                    let sqrt_expr = SqrtExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Sqrt,
                        inner: Box::new(sqrt_expr.ok_or_else(|| anyhow::anyhow!("Expected SqrtExpr"))?),
                    })
                },
                Rule::exp_fn => {
                    let exp_expr = ExpExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Exp,
                        inner: Box::new(exp_expr.ok_or_else(|| anyhow::anyhow!("Expected ExpExpr"))?),
                    })
                },
                Rule::tan_fn => {
                    let tan_expr = TanExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Tan,
                        inner: Box::new(tan_expr.ok_or_else(|| anyhow::anyhow!("Expected TanExpr"))?),
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
                Rule::pow => {
                    let left = left.context("...while transpiling base of `pow` expression")?;
                    let right = right.context("...while transpiling exponent of `pow` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Pow,
                        inner: Box::new(PowExpr {
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
                        let mut left = left;
                        if left.inner.extend_concat(right) {
                            Ok(left)
                        } else {
                            bail!("Failed to extend Concat expression - internal inconsistency")
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
impl TranspileableExpr for Expr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        self.inner.type_of(context)
    }
    fn discover_expr_function_calls(
        &self,
        resolver: &mut TypeResolver,
        ctx: &ScopeContext,
    ) -> Option<Result<()>> {
        self.inner.discover_expr_function_calls(resolver, ctx)
    }
    fn build_expr_recipe(
        &self,
        resolver: &TypeResolver,
        ctx: &ScopeContext,
        deps: &mut HashSet<TypeSlot>,
    ) -> Option<ExprRecipe> {
        self.inner.build_expr_recipe(resolver, ctx, deps)
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