//! # LST Function (String Memory Estimate)
//!
//! Returns the string memory size estimate. This is a COSY INFINITY
//! compatibility function — Rosy does not require memory management,
//! but the function is provided so legacy scripts parse correctly.
//!
//! In practice, `LST(n)` simply returns `n` as a `RE`.
//!
//! ## Syntax
//!
//! ```text
//! LST(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | RE | RE |

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::{BTreeSet, HashSet};
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// LST(n) — String memory size estimator (COSY compatibility).
/// Returns `n` as RE. Rosy doesn't need memory management, but returns
/// a value for backwards compatibility.
#[derive(Debug, PartialEq)]
pub struct LstExpr {
    pub expr: Box<Expr>,
}

impl FromRule for LstExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::lst, "Expected lst rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `LST`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `LST`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `LST`"))?);
        Ok(Some(LstExpr { expr }))
    }
}
impl Transpile for LstExpr {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_output = self.expr.transpile(context)?;
        
        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(inner_output.requested_variables);

        let serialization = format!(
            "&mut RosyLST::rosy_lst(&*{})",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TranspileableExpr for LstExpr {
    fn type_of(&self, _context: &TranspilationInputContext) -> Result<RosyType> {
        Ok(RosyType::RE())
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::RE()))
    }
}
