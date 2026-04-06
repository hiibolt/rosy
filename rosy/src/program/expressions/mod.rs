//! # Expressions
//!
//! Everything in Rosy that produces a value — operators, functions, literals,
//! and variable references.
//!
//! ## Looking for something?
//!
//! | I want to... | Go to |
//! |--------------|-------|
//! | Use `+`, `-`, `*`, `/` | **[`operators::arithmetic`]** |
//! | Compare with `=`, `<`, `>`, etc. | **[`operators::comparison`]** |
//! | Use `&` (concat), `\|` (extract), `%` (derive) | **[`operators::collection`]** |
//! | Use `AND`, `OR` | **[`operators::logical`]** |
//! | Use `NOT` or unary `-` | **[`operators::unary`]** |
//! | Call `SIN`, `COS`, `TAN`, ... | **[`functions::math::trig`]** |
//! | Call `EXP`, `LOG`, `SQRT`, `SQR`, `^` | **[`functions::math::exponential`]** |
//! | Call `CMPLX`, `CONJ`, `REAL`, `IMAG` | **[`functions::math::complex`]** |
//! | Call `ABS`, `INT`, `NINT`, `NORM`, `CONS` | **[`functions::math::rounding`]** |
//! | Call `VMIN`, `VMAX` | **[`functions::math::vector`]** |
//! | Call `TYPE`, `ISRT`, `ISRT3` | **[`functions::math::query`]** |
//! | Convert types with `ST()`, `CM()`, `RE()`, `LO()`, `VE()` | **[`functions::conversion`]** |
//! | Use `LENGTH`, `TRIM`, `LTRIM`, `POSITION` | **[`functions::sys`]** |
//! | Write a literal number, string, or boolean | **[`types`]** |
//! | Construct `DA(n)` or `CD(n)` | **[`types::da`]**, **[`types::cd`]** |
//!
//! ## Example
//!
//! ```text
//! x + y * 2           { arithmetic with precedence }
//! 1 & 2 & 3           { vector concatenation }
//! vec|3               { extract 3rd element }
//! SIN(x)              { intrinsic function }
//! ST(42)              { type conversion }
//! DA(1)               { DA variable constructor }
//! ```

pub mod core;
pub mod functions;
pub mod operators;
pub mod types;

use std::collections::HashSet;
use crate::{ast::{FromRule, PRATT_PARSER, Rule}, resolve::{ExprRecipe, ScopeContext, TypeResolver, TypeSlot}, rosy_lib::RosyType, transpile::{TranspileableExpr, ExprFunctionCallResult, ConcatExtensionResult, add_context_to_all}};
use crate::transpile::{Transpile, TranspilationInputContext, TranspilationOutput};

use crate::program::expressions::core::var_expr::VarExpr;

use crate::program::expressions::functions::conversion::complex_convert::ComplexConvertExpr;
use crate::program::expressions::functions::conversion::string_convert::StringConvertExpr;
use crate::program::expressions::functions::conversion::logical_convert::LogicalConvertExpr;
use crate::program::expressions::functions::conversion::re_convert::ReConvertExpr;
use crate::program::expressions::functions::conversion::ve_convert::VeConvertExpr;
use crate::program::expressions::functions::math::trig::tan::TanExpr;
use crate::program::expressions::functions::math::trig::sin::SinExpr;
use crate::program::expressions::functions::math::trig::cos::CosExpr;
use crate::program::expressions::functions::math::trig::asin::AsinExpr;
use crate::program::expressions::functions::math::trig::acos::AcosExpr;
use crate::program::expressions::functions::math::trig::atan::AtanExpr;
use crate::program::expressions::functions::math::trig::sinh::SinhExpr;
use crate::program::expressions::functions::math::trig::cosh::CoshExpr;
use crate::program::expressions::functions::math::trig::tanh::TanhExpr;
use crate::program::expressions::functions::math::exponential::sqr::SqrExpr;
use crate::program::expressions::functions::math::exponential::sqrt::SqrtExpr;
use crate::program::expressions::functions::math::exponential::exp::ExpExpr;
use crate::program::expressions::functions::math::exponential::log::LogExpr;
use crate::program::expressions::functions::math::exponential::pow::PowExpr;
use crate::program::expressions::functions::math::complex::cmplx::CmplxExpr;
use crate::program::expressions::functions::math::complex::conj::ConjExpr;
use crate::program::expressions::functions::math::complex::real_fn::RealFnExpr;
use crate::program::expressions::functions::math::complex::imag_fn::ImagFnExpr;
use crate::program::expressions::functions::math::rounding::abs::AbsExpr;
use crate::program::expressions::functions::math::rounding::norm::NormExpr;
use crate::program::expressions::functions::math::rounding::cons::ConsExpr;
use crate::program::expressions::functions::math::rounding::int_fn::IntExpr;
use crate::program::expressions::functions::math::rounding::nint::NintExpr;
use crate::program::expressions::functions::math::vector::vmax::VmaxExpr;
use crate::program::expressions::functions::math::vector::vmin::VminExpr;
use crate::program::expressions::functions::math::query::type_fn::TypeFnExpr;
use crate::program::expressions::functions::math::query::isrt::IsrtExpr;
use crate::program::expressions::functions::math::query::isrt3::Isrt3Expr;
use crate::program::expressions::functions::math::memory::lst::LstExpr;
use crate::program::expressions::functions::math::memory::lcm::LcmExpr;
use crate::program::expressions::functions::math::memory::lcd::LcdExpr;
use crate::program::expressions::functions::math::memory::lre::LreExpr;
use crate::program::expressions::functions::math::memory::llo::LloExpr;
use crate::program::expressions::functions::math::memory::lve::LveExpr;
use crate::program::expressions::functions::math::memory::lda::LdaExpr;
use crate::program::expressions::functions::sys::trim::TrimExpr;
use crate::program::expressions::functions::sys::ltrim::LtrimExpr;
use crate::program::expressions::functions::sys::varmem::VarmemExpr;
use crate::program::expressions::functions::sys::varpoi::VarpoiExpr;

use crate::program::expressions::operators::arithmetic::add::AddExpr;
use crate::program::expressions::operators::arithmetic::sub::SubExpr;
use crate::program::expressions::operators::arithmetic::mult::MultExpr;
use crate::program::expressions::operators::arithmetic::div::DivExpr;
use crate::program::expressions::operators::comparison::eq::EqExpr;
use crate::program::expressions::operators::comparison::neq::NeqExpr;
use crate::program::expressions::operators::comparison::lt::LtExpr;
use crate::program::expressions::operators::comparison::gt::GtExpr;
use crate::program::expressions::operators::comparison::lte::LteExpr;
use crate::program::expressions::operators::comparison::gte::GteExpr;
use crate::program::expressions::operators::unary::not::NotExpr;
use crate::program::expressions::operators::unary::neg::NegExpr;
use crate::program::expressions::operators::collection::concat::ConcatExpr;
use crate::program::expressions::operators::collection::extract::ExtractExpr;
use crate::program::expressions::operators::collection::derive::DeriveExpr;
use crate::program::expressions::operators::logical::and_op::AndExpr;
use crate::program::expressions::operators::logical::or_op::OrExpr;

use crate::program::expressions::functions::sys::length::LengthExpr;
use crate::program::expressions::functions::sys::position::PositionExpr;
use crate::program::expressions::functions::math::special::erf::ErfExpr;
use crate::program::expressions::functions::math::special::werf::WerfExpr;

use crate::program::expressions::types::da::DAExpr;
use crate::program::expressions::types::cd::CDExpr;
use anyhow::{Context, Error, Result, bail};

use crate::program::statements::SourceLocation;

#[derive(Debug)]
pub struct Expr {
    pub enum_variant: ExprEnum,
    pub inner: Box<dyn TranspileableExpr>,
    pub source_location: SourceLocation,
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
    Cos,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Sqr,
    Sqrt,
    Exp,
    Log,
    Tan,
    Vmax,
    Vmin,
    Abs,
    Norm,
    Cons,
    Int,
    Nint,
    TypeFn,
    Trim,
    Ltrim,
    Isrt,
    Isrt3,
    Cmplx,
    Conj,
    Lst,
    Lcm,
    Lcd,
    Lre,
    Llo,
    Lve,
    Lda,
    Neg,
    Derive,
    RealFn,
    ImagFn,
    ReConvert,
    VeConvert,
    Varmem,
    Varpoi,
    Erf,
    Werf,
    And,
    Or,
    Position,
}

impl FromRule for Expr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Expr>> {
        let result = PRATT_PARSER
            .map_primary(|primary| {
                let loc = SourceLocation::from_pair(&primary);
                match primary.as_rule() {
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
                        source_location: loc,
                    })
                },
                Rule::not_expr => {
                    // NOT expression: parse the inner operand
                    // The inner can be: boolean, variable_identifier, or expr (parenthesized)
                    let not_loc = loc.clone();
                    let mut inner = primary.into_inner();
                    let operand_pair = inner.next()
                        .ok_or_else(|| anyhow::anyhow!("NOT expression missing operand"))?;

                    // Handle different operand types
                    let operand = match operand_pair.as_rule() {
                        Rule::boolean => {
                            let op_loc = SourceLocation::from_pair(&operand_pair);
                            let b = bool::from_rule(operand_pair)?
                                .ok_or_else(|| anyhow::anyhow!("Expected boolean"))?;
                            Expr {
                                enum_variant: ExprEnum::Boolean,
                                inner: Box::new(b),
                                source_location: op_loc,
                            }
                        },
                        Rule::variable_identifier => {
                            let op_loc = SourceLocation::from_pair(&operand_pair);
                            let var_expr = VarExpr::from_rule(operand_pair)?
                                .ok_or_else(|| anyhow::anyhow!("Expected VarExpr"))?;
                            Expr {
                                enum_variant: ExprEnum::Var,
                                inner: Box::new(var_expr),
                                source_location: op_loc,
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
                        source_location: not_loc,
                    })
                },
                Rule::variable_identifier => {
                    let var_expr = VarExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Var,
                        inner: Box::new(var_expr.ok_or_else(|| anyhow::anyhow!("Expected VarExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::number => {
                    let n = f64::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Number,
                        inner: Box::new(n.ok_or_else(|| anyhow::anyhow!("Expected number"))?),
                        source_location: loc.clone(),
                    })
                }
                Rule::boolean => {
                    let b = bool::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Boolean,
                        inner: Box::new(b.ok_or_else(|| anyhow::anyhow!("Expected boolean"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::string => {
                    let s = String::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::String,
                        inner: Box::new(s.ok_or_else(|| anyhow::anyhow!("Expected string"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::cm => {
                    let cm_expr = ComplexConvertExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Complex,
                        inner: Box::new(cm_expr.ok_or_else(|| anyhow::anyhow!("Expected ComplexConvertExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::st => {
                    let st_expr = StringConvertExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::StringConvert,
                        inner: Box::new(st_expr.ok_or_else(|| anyhow::anyhow!("Expected StringConvertExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::lo => {
                    let lo_expr = LogicalConvertExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::LogicalConvert,
                        inner: Box::new(lo_expr.ok_or_else(|| anyhow::anyhow!("Expected LogicalConvertExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::da => {
                    let da_expr = DAExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::DA,
                        inner: Box::new(da_expr.ok_or_else(|| anyhow::anyhow!("Expected DAExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::cd_intrinsic => {
                    let cd_expr = CDExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::CD,
                        inner: Box::new(cd_expr.ok_or_else(|| anyhow::anyhow!("Expected CDExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::position => {
                    let position_expr = PositionExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Position,
                        inner: Box::new(position_expr.ok_or_else(|| anyhow::anyhow!("Expected PositionExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::length => {
                    let length_expr = LengthExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Length,
                        inner: Box::new(length_expr.ok_or_else(|| anyhow::anyhow!("Expected LengthExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::sin => {
                    let sin_expr = SinExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Sin,
                        inner: Box::new(sin_expr.ok_or_else(|| anyhow::anyhow!("Expected SinExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::cos_fn => {
                    let cos_expr = CosExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Cos,
                        inner: Box::new(cos_expr.ok_or_else(|| anyhow::anyhow!("Expected CosExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::asin_fn => {
                    let asin_expr = AsinExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Asin,
                        inner: Box::new(asin_expr.ok_or_else(|| anyhow::anyhow!("Expected AsinExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::acos_fn => {
                    let acos_expr = AcosExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Acos,
                        inner: Box::new(acos_expr.ok_or_else(|| anyhow::anyhow!("Expected AcosExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::atan_fn => {
                    let atan_expr = AtanExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Atan,
                        inner: Box::new(atan_expr.ok_or_else(|| anyhow::anyhow!("Expected AtanExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::sinh_fn => {
                    let sinh_expr = SinhExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Sinh,
                        inner: Box::new(sinh_expr.ok_or_else(|| anyhow::anyhow!("Expected SinhExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::cosh_fn => {
                    let cosh_expr = CoshExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Cosh,
                        inner: Box::new(cosh_expr.ok_or_else(|| anyhow::anyhow!("Expected CoshExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::tanh_fn => {
                    let tanh_expr = TanhExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Tanh,
                        inner: Box::new(tanh_expr.ok_or_else(|| anyhow::anyhow!("Expected TanhExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::sqr => {
                    let sqr_expr = SqrExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Sqr,
                        inner: Box::new(sqr_expr.ok_or_else(|| anyhow::anyhow!("Expected SqrExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::sqrt_fn => {
                    let sqrt_expr = SqrtExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Sqrt,
                        inner: Box::new(sqrt_expr.ok_or_else(|| anyhow::anyhow!("Expected SqrtExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::exp_fn => {
                    let exp_expr = ExpExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Exp,
                        inner: Box::new(exp_expr.ok_or_else(|| anyhow::anyhow!("Expected ExpExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::log_fn => {
                    let log_expr = LogExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Log,
                        inner: Box::new(log_expr.ok_or_else(|| anyhow::anyhow!("Expected LogExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::tan_fn => {
                    let tan_expr = TanExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Tan,
                        inner: Box::new(tan_expr.ok_or_else(|| anyhow::anyhow!("Expected TanExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::vmax => {
                    let vmax_expr = VmaxExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Vmax,
                        inner: Box::new(vmax_expr.ok_or_else(|| anyhow::anyhow!("Expected VmaxExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::lst => {
                    let lst_expr = LstExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Lst,
                        inner: Box::new(lst_expr.ok_or_else(|| anyhow::anyhow!("Expected LstExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::lcm => {
                    let lcm_expr = LcmExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Lcm,
                        inner: Box::new(lcm_expr.ok_or_else(|| anyhow::anyhow!("Expected LcmExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::lcd => {
                    let lcd_expr = LcdExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Lcd,
                        inner: Box::new(lcd_expr.ok_or_else(|| anyhow::anyhow!("Expected LcdExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::lre => {
                    let lre_expr = LreExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Lre,
                        inner: Box::new(lre_expr.ok_or_else(|| anyhow::anyhow!("Expected LreExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::llo => {
                    let llo_expr = LloExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Llo,
                        inner: Box::new(llo_expr.ok_or_else(|| anyhow::anyhow!("Expected LloExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::lve => {
                    let lve_expr = LveExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Lve,
                        inner: Box::new(lve_expr.ok_or_else(|| anyhow::anyhow!("Expected LveExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::lda => {
                    let lda_expr = LdaExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Lda,
                        inner: Box::new(lda_expr.ok_or_else(|| anyhow::anyhow!("Expected LdaExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::vmin => {
                    let vmin_expr = VminExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Vmin,
                        inner: Box::new(vmin_expr.ok_or_else(|| anyhow::anyhow!("Expected VminExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::abs_fn => {
                    let abs_expr = AbsExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Abs,
                        inner: Box::new(abs_expr.ok_or_else(|| anyhow::anyhow!("Expected AbsExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::norm_fn => {
                    let norm_expr = NormExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Norm,
                        inner: Box::new(norm_expr.ok_or_else(|| anyhow::anyhow!("Expected NormExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::cons_fn => {
                    let cons_expr = ConsExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Cons,
                        inner: Box::new(cons_expr.ok_or_else(|| anyhow::anyhow!("Expected ConsExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::int_fn => {
                    let int_expr = IntExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Int,
                        inner: Box::new(int_expr.ok_or_else(|| anyhow::anyhow!("Expected IntExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::nint_fn => {
                    let nint_expr = NintExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Nint,
                        inner: Box::new(nint_expr.ok_or_else(|| anyhow::anyhow!("Expected NintExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::type_fn => {
                    let type_fn_expr = TypeFnExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::TypeFn,
                        inner: Box::new(type_fn_expr.ok_or_else(|| anyhow::anyhow!("Expected TypeFnExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::trim_fn => {
                    let trim_expr = TrimExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Trim,
                        inner: Box::new(trim_expr.ok_or_else(|| anyhow::anyhow!("Expected TrimExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::ltrim_fn => {
                    let ltrim_expr = LtrimExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Ltrim,
                        inner: Box::new(ltrim_expr.ok_or_else(|| anyhow::anyhow!("Expected LtrimExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::isrt_fn => {
                    let isrt_expr = IsrtExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Isrt,
                        inner: Box::new(isrt_expr.ok_or_else(|| anyhow::anyhow!("Expected IsrtExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::isrt3_fn => {
                    let isrt3_expr = Isrt3Expr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Isrt3,
                        inner: Box::new(isrt3_expr.ok_or_else(|| anyhow::anyhow!("Expected Isrt3Expr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::cmplx_fn => {
                    let cmplx_expr = CmplxExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Cmplx,
                        inner: Box::new(cmplx_expr.ok_or_else(|| anyhow::anyhow!("Expected CmplxExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::conj_fn => {
                    let conj_expr = ConjExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Conj,
                        inner: Box::new(conj_expr.ok_or_else(|| anyhow::anyhow!("Expected ConjExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::real_fn => {
                    let real_fn_expr = RealFnExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::RealFn,
                        inner: Box::new(real_fn_expr.ok_or_else(|| anyhow::anyhow!("Expected RealFnExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::imag_fn => {
                    let imag_fn_expr = ImagFnExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::ImagFn,
                        inner: Box::new(imag_fn_expr.ok_or_else(|| anyhow::anyhow!("Expected ImagFnExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::re_fn => {
                    let re_convert_expr = ReConvertExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::ReConvert,
                        inner: Box::new(re_convert_expr.ok_or_else(|| anyhow::anyhow!("Expected ReConvertExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::ve_fn => {
                    let ve_convert_expr = VeConvertExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::VeConvert,
                        inner: Box::new(ve_convert_expr.ok_or_else(|| anyhow::anyhow!("Expected VeConvertExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::varmem => {
                    let varmem_expr = VarmemExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Varmem,
                        inner: Box::new(varmem_expr.ok_or_else(|| anyhow::anyhow!("Expected VarmemExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::varpoi => {
                    let varpoi_expr = VarpoiExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Varpoi,
                        inner: Box::new(varpoi_expr.ok_or_else(|| anyhow::anyhow!("Expected VarpoiExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::erf_fn => {
                    let erf_expr = ErfExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Erf,
                        inner: Box::new(erf_expr.ok_or_else(|| anyhow::anyhow!("Expected ErfExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::werf_fn => {
                    let werf_expr = WerfExpr::from_rule(primary)?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Werf,
                        inner: Box::new(werf_expr.ok_or_else(|| anyhow::anyhow!("Expected WerfExpr"))?),
                        source_location: loc.clone(),
                    })
                },
                Rule::expr => {
                    // handle parenthesized expressions by recursively parsing
                    Expr::from_rule(primary)
                        .context("Failed to build expression for parenthesized `expr`")?
                        .ok_or_else(|| anyhow::anyhow!("Expected expression for parenthesized `expr`"))
                },
                _ => bail!("Unexpected primary expr: {:?}", primary.as_rule()),
                }
            })
            .map_infix(|
                left,
                op,
                right
            | {
                let op_loc = SourceLocation::from_pair(&op);
                match op.as_rule() {
                Rule::add => {
                    let left = left.context("...while transpiling left-hand side of `add` expression")?;
                    let right = right.context("...while transpiling right-hand side of `add` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Add,
                        inner: Box::new(AddExpr {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    , source_location: op_loc.clone() })
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
                    , source_location: op_loc.clone() })
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
                    , source_location: op_loc.clone() })
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
                    , source_location: op_loc.clone() })
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
                    , source_location: op_loc.clone() })
                },
                Rule::concat => {
                    let left = left.context("...while transpiling left-hand side of `concat` expression")?;
                    let right = right.context("...while transpiling right-hand side of `concat` expression")?;

                    // If left is already a Concat, extend its terms instead of nesting
                    if left.enum_variant == ExprEnum::Concat {
                        let mut left = left;
                        match left.inner.extend_concat(right) {
                            ConcatExtensionResult::Extended => Ok(left),
                            ConcatExtensionResult::NotAConcatExpr => bail!("Failed to extend Concat expression - internal inconsistency"),
                        }
                    } else {
                        let terms = vec![left, right];
                        Ok(Expr {
                            enum_variant: ExprEnum::Concat,
                            inner: Box::new(ConcatExpr { terms })
                        , source_location: op_loc.clone() })
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
                    , source_location: op_loc.clone() })
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
                    , source_location: op_loc.clone() })
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
                    , source_location: op_loc.clone() })
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
                    , source_location: op_loc.clone() })
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
                    , source_location: op_loc.clone() })
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
                    , source_location: op_loc.clone() })
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
                    , source_location: op_loc.clone() })
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
                    , source_location: op_loc.clone() })
                },
                Rule::and_op => {
                    let left = left.context("...while transpiling left-hand side of `AND` expression")?;
                    let right = right.context("...while transpiling right-hand side of `AND` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::And,
                        inner: Box::new(AndExpr {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    , source_location: op_loc.clone() })
                },
                Rule::or_op => {
                    let left = left.context("...while transpiling left-hand side of `OR` expression")?;
                    let right = right.context("...while transpiling right-hand side of `OR` expression")?;
                    Ok(Expr {
                        enum_variant: ExprEnum::Or,
                        inner: Box::new(OrExpr {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    , source_location: op_loc.clone() })
                },
                _ => bail!("Unexpected infix operator: {:?}", op.as_rule()),
                }
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
    ) -> ExprFunctionCallResult {
        self.inner.discover_expr_function_calls(resolver, ctx)
    }
    fn build_expr_recipe(
        &self,
        resolver: &TypeResolver,
        ctx: &ScopeContext,
        deps: &mut HashSet<TypeSlot>,
    ) -> ExprRecipe {
        self.inner.build_expr_recipe(resolver, ctx, deps)
    }
    fn extend_concat(&mut self, right: Expr) -> ConcatExtensionResult {
        self.inner.extend_concat(right)
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