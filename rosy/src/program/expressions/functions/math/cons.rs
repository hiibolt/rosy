//! # CONS Function
//!
//! Extracts the constant (scalar) part of a value.
//!
//! ## Syntax
//!
//! ```text
//! CONS(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | RE    | RE     |
//! | CM    | CM     |
//! | VE    | RE     |
//! | DA    | RE     |
//! | CD    | CM     |

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr};
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;
use std::collections::HashSet;

/// AST node for the `CONS(expr)` intrinsic function.
#[derive(Debug, PartialEq)]
pub struct ConsExpr {
    pub expr: Box<Expr>,
}

impl FromRule for ConsExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::cons_fn, "Expected cons_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `CONS`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `CONS`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `CONS`"))?);
        Ok(Some(ConsExpr { expr }))
    }
}

impl Transpile for ConsExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_output = self.expr.transpile(context)?;

        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(inner_output.requested_variables);

        let serialization = format!(
            "&mut RosyCONS::rosy_cons(&*{})?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}

impl TranspileableExpr for ConsExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::cons;

        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in CONS")?;

        cons::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "CONS not supported for type: {:?}",
                    inner_type
                )
            })
    }

    fn discover_expr_function_calls(&self, resolver: &mut TypeResolver, ctx: &ScopeContext) -> Option<Result<()>> {
        Some(resolver.discover_expr_function_calls(&self.expr, ctx))
    }

    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        // CONS has non-uniform type mapping (VE->RE, DA->RE, RE->RE, CM->CM),
        // so we cannot represent it with ExprRecipe::Sin (which assumes type
        // preservation). Return None so the type resolver skips conflict
        // checking for explicitly-typed variables assigned from CONS.
        None
    }
}
