//! # EXP Function (Exponential)
//!
//! Computes the exponential function e^x.
//!
//! ## Syntax
//!
//! ```text
//! EXP(expr)
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
//! x := EXP(1);           { ≈ 2.71828 }
//! ```

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr, ValueKind};
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::HashSet;

/// AST node for the `EXP(expr)` intrinsic function (exponential e^x).
#[derive(Debug, PartialEq)]
pub struct ExpExpr {
    pub expr: Box<Expr>,
}

impl FromRule for ExpExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::exp_fn, "Expected exp_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `EXP`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `EXP`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `EXP`"))?);
        Ok(Some(ExpExpr { expr }))
    }
}
impl Transpile for ExpExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        // Transpile the inner expression
        let inner_output = self.expr.transpile(context)?;

        // Generate the transpiled code
        let serialization = format!("RosyEXP::rosy_exp({})?", inner_output.as_ref());

        Ok(TranspilationOutput {
            serialization,
            requested_variables: inner_output.requested_variables,
            value_kind: ValueKind::Owned,
        })
    }
}
impl TranspileableExpr for ExpExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::exp;
        
        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in EXP")?;
        
        exp::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "EXP not supported for type: {:?}",
                    inner_type
                )
            })
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        None
    }
}
