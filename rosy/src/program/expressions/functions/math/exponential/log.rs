//! # LOG Function (Natural Logarithm)
//!
//! Computes the natural logarithm ln(x).
//!
//! ## Syntax
//!
//! ```text
//! LOG(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | RE | RE |
//! | CM | CM |
//! | VE | VE |
//! | DA | DA |
//!
//! ## Example
//!
//! ```text
//! VARIABLE (RE) x;
//! x := LOG(2.71828);     { ≈ 1.0 }
//! ```

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr};
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;
use std::collections::HashSet;

/// AST node for the `LOG(expr)` intrinsic function (natural logarithm ln(x)).
#[derive(Debug, PartialEq)]
pub struct LogExpr {
    pub expr: Box<Expr>,
}

impl FromRule for LogExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::log_fn, "Expected log_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `LOG`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `LOG`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `LOG`"))?);
        Ok(Some(LogExpr { expr }))
    }
}
impl Transpile for LogExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        // Transpile the inner expression
        let inner_output = self.expr.transpile(context)?;

        // Combine requested variables
        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(inner_output.requested_variables);

        // Generate the transpiled code
        let serialization = format!(
            "&mut RosyLOG::rosy_log(&*{})?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
            ..Default::default()
        })
    }
}
impl TranspileableExpr for LogExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::log;

        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in LOG")?;

        log::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "LOG not supported for type: {:?}",
                    inner_type
                )
            })
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        None
    }
}
