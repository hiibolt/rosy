//! # LTRIM Function
//!
//! Removes leading space characters from a string.
//!
//! ## Syntax
//!
//! ```text
//! LTRIM(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | ST | ST |

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::{BTreeSet, HashSet};
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// AST node for the `LTRIM(expr)` function.
#[derive(Debug, PartialEq)]
pub struct LtrimExpr {
    pub expr: Box<Expr>,
}

impl FromRule for LtrimExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::ltrim_fn, "Expected ltrim_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `LTRIM`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `LTRIM`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `LTRIM`"))?);
        Ok(Some(LtrimExpr { expr }))
    }
}

impl Transpile for LtrimExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_output = self.expr.transpile(context)?;

        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(inner_output.requested_variables);

        let serialization = format!(
            "&mut RosyLTRIM::rosy_ltrim(&*{})?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}

impl TranspileableExpr for LtrimExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::ltrim;
        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in LTRIM")?;
        ltrim::get_return_type(&inner_type)
            .ok_or_else(|| anyhow::anyhow!("LTRIM not supported for type: {:?}", inner_type))
    }
    fn discover_expr_function_calls(&self, resolver: &mut TypeResolver, ctx: &ScopeContext) -> Option<Result<()>> {
        Some(resolver.discover_expr_function_calls(&self.expr, ctx))
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::ST()))
    }
}
