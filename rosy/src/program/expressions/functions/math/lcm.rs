//! # LCM Function (Complex Memory Estimate)
//!
//! Returns the complex-number memory size estimate. This is a COSY INFINITY
//! compatibility function — returns `2*n` as a `RE`.
//!
//! ## Syntax
//!
//! ```text
//! LCM(expr)
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

/// LCM(n) — Complex memory size estimator (COSY compatibility).
/// Returns `2*n` as RE. Rosy doesn't need memory management.
#[derive(Debug, PartialEq)]
pub struct LcmExpr {
    pub expr: Box<Expr>,
}

impl FromRule for LcmExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::lcm, "Expected lcm rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `LCM`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `LCM`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `LCM`"))?);
        Ok(Some(LcmExpr { expr }))
    }
}
impl Transpile for LcmExpr {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_output = self.expr.transpile(context)?;
        
        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(inner_output.requested_variables);

        let serialization = format!(
            "&mut RosyLCM::rosy_lcm(&*{})",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TranspileableExpr for LcmExpr {
    fn type_of(&self, _context: &TranspilationInputContext) -> Result<RosyType> {
        Ok(RosyType::RE())
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::RE()))
    }
}
