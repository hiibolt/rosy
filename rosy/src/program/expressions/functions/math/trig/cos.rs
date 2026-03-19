//! # COS Function
//!
//! Computes the cosine of a value.
//!
//! ## Syntax
//!
//! ```text
//! COS(expr)
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
//! x := COS(0);   { = 1.0 }
//! ```

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr};
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;
use std::collections::HashSet;

/// AST node for the `COS(expr)` intrinsic function.
#[derive(Debug, PartialEq)]
pub struct CosExpr {
    pub expr: Box<Expr>,
}

impl FromRule for CosExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::cos_fn, "Expected cos_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `COS`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `COS`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `COS`"))?);
        Ok(Some(CosExpr { expr }))
    }
}
impl Transpile for CosExpr {
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
            "&mut RosyCOS::rosy_cos(&*{})?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TranspileableExpr for CosExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::cos;

        // Get the type of the inner expression
        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in COS")?;

        // Use the COS registry to get the return type
        cos::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "COS not supported for type: {:?}",
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
