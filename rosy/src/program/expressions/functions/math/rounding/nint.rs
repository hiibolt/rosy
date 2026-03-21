//! # NINT Function
//!
//! Rounds a value to the nearest integer.
//!
//! ## Syntax
//!
//! ```text
//! NINT(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | RE | RE |
//! | VE | VE |

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr, ValueKind};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::HashSet;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// AST node for the `NINT(expr)` intrinsic function (round to nearest integer).
#[derive(Debug, PartialEq)]
pub struct NintExpr {
    pub expr: Box<Expr>,
}

impl FromRule for NintExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::nint_fn, "Expected nint_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `NINT`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `NINT`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `NINT`"))?);
        Ok(Some(NintExpr { expr }))
    }
}
impl Transpile for NintExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_output = self.expr.transpile(context)?;

        let serialization = format!("RosyNINT::rosy_nint({})?", inner_output.as_ref());

        Ok(TranspilationOutput {
            serialization,
            requested_variables: inner_output.requested_variables,
            value_kind: ValueKind::Owned,
        })
    }
}
impl TranspileableExpr for NintExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::nint;

        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in NINT")?;

        nint::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "NINT not supported for type: {:?}",
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
