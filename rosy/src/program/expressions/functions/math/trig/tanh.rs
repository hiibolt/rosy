//! # TANH Function
//!
//! Computes the hyperbolic tangent of a value.
//!
//! ## Syntax
//!
//! ```text
//! TANH(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | RE | RE |
//! | VE | VE |
//! | DA | DA |
//!
//! Note: CM is NOT supported for TANH in COSY.

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr, ValueKind};
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::HashSet;

/// AST node for the `TANH(expr)` intrinsic function.
#[derive(Debug, PartialEq)]
pub struct TanhExpr {
    pub expr: Box<Expr>,
}

impl FromRule for TanhExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::tanh_fn, "Expected tanh_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `TANH`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `TANH`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `TANH`"))?);
        Ok(Some(TanhExpr { expr }))
    }
}
impl Transpile for TanhExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_type = self.expr.type_of(context).map_err(|e| vec![e])?;

        let inner_output = self.expr.transpile(context)?;

        let serialization = if inner_type == RosyType::RE() {
            format!("{}.tanh()", inner_output.as_value())
        } else {
            format!("RosyTANH::rosy_tanh({})?", inner_output.as_ref())
        };

        Ok(TranspilationOutput {
            serialization,
            requested_variables: inner_output.requested_variables,
            value_kind: ValueKind::Owned,
        })
    }
}
impl TranspileableExpr for TanhExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::tanh;

        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in TANH")?;

        tanh::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "TANH not supported for type: {:?}",
                    inner_type
                )
            })
    }
    fn discover_expr_function_calls(&self, resolver: &mut TypeResolver, ctx: &ScopeContext) -> Option<Result<()>> {
        Some(resolver.discover_expr_function_calls(&self.expr, ctx))
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        None
    }
}
