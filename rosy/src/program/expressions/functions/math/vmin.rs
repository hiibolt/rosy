//! # VMIN Function
//!
//! Returns the minimum element of a vector.
//!
//! ## Syntax
//!
//! ```text
//! VMIN(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | VE | RE |

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::{BTreeSet, HashSet};
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// AST node for the `VMIN(expr)` function (vector minimum).
#[derive(Debug, PartialEq)]
pub struct VminExpr {
    pub expr: Box<Expr>,
}

impl FromRule for VminExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::vmin, "Expected vmin rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `VMIN`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `VMIN`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `VMIN`"))?);
        Ok(Some(VminExpr { expr }))
    }
}
impl Transpile for VminExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_output = self.expr.transpile(context)?;

        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(inner_output.requested_variables);

        let serialization = format!(
            "&mut RosyVMIN::rosy_vmin(&*{})?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TranspileableExpr for VminExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in VMIN")?;

        match inner_type {
            t if t == RosyType::VE() => Ok(RosyType::RE()),
            _ => anyhow::bail!(
                "VMIN not supported for type: {:?}. Only VE is supported.",
                inner_type
            ),
        }
    }
    fn discover_expr_function_calls(&self, resolver: &mut TypeResolver, ctx: &ScopeContext) -> Option<Result<()>> {
        Some(resolver.discover_expr_function_calls(&self.expr, ctx))
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::RE()))
    }
}
