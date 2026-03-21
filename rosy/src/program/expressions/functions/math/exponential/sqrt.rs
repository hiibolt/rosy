//! # SQRT Function (Square Root)
//!
//! Computes the square root of x.
//!
//! ## Syntax
//!
//! ```text
//! SQRT(expr)
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
//! x := SQRT(4);           { = 2.0 }
//! ```

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr};
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;
use std::collections::HashSet;

/// AST node for the `SQRT(expr)` intrinsic function (square root).
#[derive(Debug, PartialEq)]
pub struct SqrtExpr {
    pub expr: Box<Expr>,
}

impl FromRule for SqrtExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::sqrt_fn, "Expected sqrt_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `SQRT`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `SQRT`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `SQRT`"))?);
        Ok(Some(SqrtExpr { expr }))
    }
}
impl Transpile for SqrtExpr {
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
            "&mut RosySQRT::rosy_sqrt(&*{})?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
            ..Default::default()
        })
    }
}
impl TranspileableExpr for SqrtExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::sqrt;

        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in SQRT")?;

        sqrt::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "SQRT not supported for type: {:?}",
                    inner_type
                )
            })
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        None
    }
}
