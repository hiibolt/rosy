//! # VMAX Function
//!
//! Returns the maximum element of a vector.
//!
//! ## Syntax
//!
//! ```text
//! VMAX(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | VE | RE |
//!
//! ## Example
//!
//! ```text
//! VARIABLE (VE) v;
//! VARIABLE (RE) m;
//! v := 3 & 1 & 4 & 1 & 5;
//! m := VMAX(v);          { 5.0 }
//! ```

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr, ValueKind};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::HashSet;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// AST node for the `VMAX(expr)` function (vector maximum).
#[derive(Debug, PartialEq)]
pub struct VmaxExpr {
    pub expr: Box<Expr>,
}

impl FromRule for VmaxExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::vmax, "Expected vmax rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `VMAX`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `VMAX`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `VMAX`"))?);
        Ok(Some(VmaxExpr { expr }))
    }
}
impl Transpile for VmaxExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_output = self.expr.transpile(context)?;

        let serialization = format!("RosyVMAX::rosy_vmax({})?", inner_output.as_ref());

        Ok(TranspilationOutput {
            serialization,
            requested_variables: inner_output.requested_variables,
            value_kind: ValueKind::Owned,
        })
    }
}
impl TranspileableExpr for VmaxExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in VMAX")?;

        match inner_type {
            t if t == RosyType::VE() => Ok(RosyType::RE()),
            _ => anyhow::bail!(
                "VMAX not supported for type: {:?}. Only VE is supported.",
                inner_type
            ),
        }
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::RE()))
    }
}
