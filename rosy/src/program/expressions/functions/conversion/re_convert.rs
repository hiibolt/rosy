//! # RE() — Real Number Conversion
//!
//! Converts a value to a real number (`RE`).
//!
//! ## Syntax
//!
//! ```text
//! RE(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | RE    | RE     |
//! | ST    | RE     |
//! | CM    | RE     |
//! | VE    | RE     |
//! | DA    | RE     |

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::TranspileableExpr;
use crate::transpile::{Transpile, TranspilationInputContext, TranspilationOutput, ValueKind};
use anyhow::{Result, Error, Context};
use std::collections::HashSet;
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// AST node for the `RE(expr)` type conversion function.
#[derive(Debug, PartialEq)]
pub struct ReConvertExpr {
    pub expr: Box<Expr>,
}

impl FromRule for ReConvertExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::re_fn, "Expected re_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `RE`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `RE`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `RE`"))?);
        Ok(Some(ReConvertExpr { expr }))
    }
}

impl TranspileableExpr for ReConvertExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        let expr_type = self.expr.type_of(context)
            .map_err(|e| e.context("...while determining type of expression for RE conversion"))?;
        let result_type = crate::rosy_lib::intrinsics::re_convert::get_return_type(&expr_type)
            .ok_or(anyhow::anyhow!(
                "Cannot convert type '{}' to 'RE'!",
                expr_type
            ))?;
        Ok(result_type)
    }
    fn discover_expr_function_calls(&self, resolver: &mut TypeResolver, ctx: &ScopeContext) -> Option<Result<()>> {
        Some(resolver.discover_expr_function_calls(&self.expr, ctx))
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::RE()))
    }
}

impl Transpile for ReConvertExpr {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        // Verify type compatibility
        let _ = self.type_of(context)
            .map_err(|e| vec![e.context("...while verifying types of RE conversion expression")])?;

        let inner_output = self.expr.transpile(context)
            .map_err(|e| e.into_iter().map(|err| {
                err.context("...while transpiling expression for RE conversion")
            }).collect::<Vec<Error>>())?;

        let serialization = format!(
            "RosyREConvert::rosy_re_convert({}).context(\"...while trying to convert to (RE)\")?",
            inner_output.as_ref()
        );
        Ok(TranspilationOutput {
            serialization,
            requested_variables: inner_output.requested_variables,
            value_kind: ValueKind::Owned,
        })
    }
}
