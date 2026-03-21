//! # TRIM Function
//!
//! Removes trailing space characters from a string.
//!
//! ## Syntax
//!
//! ```text
//! TRIM(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | ST | ST |

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr, ValueKind};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::HashSet;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// AST node for the `TRIM(expr)` function.
#[derive(Debug, PartialEq)]
pub struct TrimExpr {
    pub expr: Box<Expr>,
}

impl FromRule for TrimExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::trim_fn, "Expected trim_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `TRIM`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `TRIM`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `TRIM`"))?);
        Ok(Some(TrimExpr { expr }))
    }
}

impl Transpile for TrimExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_output = self.expr.transpile(context)?;

        let serialization = format!("RosyTRIM::rosy_trim({})?", inner_output.as_ref());

        Ok(TranspilationOutput {
            serialization,
            requested_variables: inner_output.requested_variables,
            value_kind: ValueKind::Owned,
        })
    }
}

impl TranspileableExpr for TrimExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::trim;
        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in TRIM")?;
        trim::get_return_type(&inner_type)
            .ok_or_else(|| anyhow::anyhow!("TRIM not supported for type: {:?}", inner_type))
    }
    fn discover_expr_function_calls(&self, resolver: &mut TypeResolver, ctx: &ScopeContext) -> Option<Result<()>> {
        Some(resolver.discover_expr_function_calls(&self.expr, ctx))
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::ST()))
    }
}
