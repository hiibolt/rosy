//! # INT Function
//!
//! Truncates a value toward zero.
//!
//! ## Syntax
//!
//! ```text
//! INT(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | RE | RE |
//! | VE | VE |
//!
//! ```rosy_test_raw
//! --- rosy ---
//! BEGIN;
//!     VARIABLE (RE) X;
//!     X := INT(2.9);
//!     WRITE 6 X;
//! END;
//! --- fox ---
//! BEGIN;
//! PROCEDURE RUN;
//!     VARIABLE X 1;
//!     X := INT(2.9);
//!     WRITE 6 X;
//! ENDPROCEDURE;
//! RUN;
//! END;
//! ```

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr, ValueKind};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::HashSet;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// AST node for the `INT(expr)` intrinsic function (truncate toward zero).
#[derive(Debug, PartialEq)]
pub struct IntExpr {
    pub expr: Box<Expr>,
}

impl FromRule for IntExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::int_fn, "Expected int_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `INT`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `INT`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `INT`"))?);
        Ok(Some(IntExpr { expr }))
    }
}
impl Transpile for IntExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_type = self.expr.type_of(context).map_err(|e| vec![e])?;

        let inner_output = self.expr.transpile(context)?;

        let serialization = if inner_type == RosyType::RE() {
            format!("{}.trunc()", inner_output.as_value())
        } else {
            format!("RosyINT::rosy_int({})?", inner_output.as_ref())
        };

        Ok(TranspilationOutput {
            serialization,
            requested_variables: inner_output.requested_variables,
            value_kind: ValueKind::Owned,
        })
    }
}
impl TranspileableExpr for IntExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::int_fn;

        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in INT")?;

        int_fn::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "INT not supported for type: {:?}",
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
