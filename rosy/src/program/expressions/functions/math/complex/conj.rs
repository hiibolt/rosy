//! # CONJ Function (Complex Conjugate)
//!
//! Computes the complex conjugate of a value.
//!
//! ## Syntax
//!
//! ```text
//! CONJ(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | RE | RE |
//! | CM | CM |
//! | CD | CD |

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr};
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;
use std::collections::HashSet;

/// AST node for the `CONJ(expr)` intrinsic function.
#[derive(Debug, PartialEq)]
pub struct ConjExpr {
    pub expr: Box<Expr>,
}

impl FromRule for ConjExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::conj_fn, "Expected conj_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `CONJ`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `CONJ`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `CONJ`"))?);
        Ok(Some(ConjExpr { expr }))
    }
}
impl Transpile for ConjExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_output = self.expr.transpile(context)?;

        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(inner_output.requested_variables);

        let serialization = format!(
            "&mut RosyCONJ::rosy_conj(&*{})?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TranspileableExpr for ConjExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::conj;

        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in CONJ")?;

        conj::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "CONJ not supported for type: {:?}",
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
