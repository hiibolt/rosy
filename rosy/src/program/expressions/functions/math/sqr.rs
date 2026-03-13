//! # SQR Function (Square Root)
//!
//! Computes the square root of a value.
//!
//! ## Syntax
//!
//! ```text
//! SQR(expr)
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
//! x := SQR(16);          { 4.0 }
//! ```

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr};
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;
use std::collections::HashSet;

/// AST node for the `SQR(expr)` intrinsic function (square root).
#[derive(Debug, PartialEq)]
pub struct SqrExpr {
    pub expr: Box<Expr>,
}

impl FromRule for SqrExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::sqr, "Expected sqr rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `SQR`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `SQR`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `SQR`"))?);
        Ok(Some(SqrExpr { expr }))
    }
}
impl Transpile for SqrExpr {
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
            "&mut RosySQR::rosy_sqr(&*{})?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TranspileableExpr for SqrExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::sqr;
        
        // Get the type of the inner expression
        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in SQR")?;
        
        // Use the SQR registry to get the return type
        sqr::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "SQR not supported for type: {:?}",
                    inner_type
                )
            })
    }
    fn build_expr_recipe(&self, resolver: &TypeResolver, ctx: &ScopeContext, deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        let inner = resolver.build_expr_recipe(&self.expr, ctx, deps);
        Some(ExprRecipe::Sin(Box::new(inner)))
    }
}
