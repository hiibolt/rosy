//! # NORM Function
//!
//! Computes the norm of a value.
//!
//! ## Syntax
//!
//! ```text
//! NORM(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | VE    | VE     |
//! | DA    | RE     |
//! | CD    | RE     |

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr};
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;
use std::collections::HashSet;

/// AST node for the `NORM(expr)` intrinsic function.
#[derive(Debug, PartialEq)]
pub struct NormExpr {
    pub expr: Box<Expr>,
}

impl FromRule for NormExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::norm_fn, "Expected norm_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `NORM`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `NORM`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `NORM`"))?);
        Ok(Some(NormExpr { expr }))
    }
}

impl Transpile for NormExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_output = self.expr.transpile(context)?;

        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(inner_output.requested_variables);

        let serialization = format!(
            "&mut RosyNORM::rosy_norm(&*{})?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}

impl TranspileableExpr for NormExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::norm;

        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in NORM")?;

        norm::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "NORM not supported for type: {:?}",
                    inner_type
                )
            })
    }

    fn discover_expr_function_calls(&self, resolver: &mut TypeResolver, ctx: &ScopeContext) -> Option<Result<()>> {
        Some(resolver.discover_expr_function_calls(&self.expr, ctx))
    }

    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        // NORM has non-uniform type mapping (DA->RE, VE->VE), so we cannot
        // represent it with ExprRecipe::Sin (which assumes type preservation).
        // Return None so the type resolver skips conflict checking for
        // explicitly-typed variables assigned from NORM.
        None
    }
}
