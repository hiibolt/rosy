//! # TYPE Function
//!
//! Returns the COSY type code of a value as RE.
//!
//! ## Syntax
//!
//! ```text
//! TYPE(expr)
//! ```
//!
//! ## Type Codes
//!
//! | Input | Code |
//! |-------|------|
//! | RE    |  1   |
//! | CM    |  2   |
//! | CD    |  3   |
//! | ST    |  4   |
//! | LO    |  5   |
//! | VE    |  6   |
//! | DA    |  8   |

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr};
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;
use std::collections::HashSet;

/// AST node for the `TYPE(expr)` intrinsic function.
#[derive(Debug, PartialEq)]
pub struct TypeFnExpr {
    pub expr: Box<Expr>,
}

impl FromRule for TypeFnExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::type_fn, "Expected type_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `TYPE`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `TYPE`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `TYPE`"))?);
        Ok(Some(TypeFnExpr { expr }))
    }
}

impl Transpile for TypeFnExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_output = self.expr.transpile(context)?;

        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(inner_output.requested_variables);

        let serialization = format!(
            "&mut RosyTYPE::rosy_type(&*{})?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}

impl TranspileableExpr for TypeFnExpr {
    fn type_of(&self, _context: &TranspilationInputContext) -> Result<RosyType> {
        // TYPE always returns RE regardless of input
        Ok(RosyType::RE())
    }

    fn discover_expr_function_calls(&self, resolver: &mut TypeResolver, ctx: &ScopeContext) -> Option<Result<()>> {
        Some(resolver.discover_expr_function_calls(&self.expr, ctx))
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::RE()))
    }
}
