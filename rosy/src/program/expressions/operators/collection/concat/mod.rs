//! # Concatenation Operator (`&`)
//!
//! Concatenates scalars and vectors into larger vectors, or strings together.
//!
//! ## Syntax
//!
//! ```text
//! expr & expr
//! ```
//!
//! ## Type Compatibility
//!
//! | Left | Right | Result | Comment |
//! |------|-------|--------|---------|
//! | RE | RE | VE | Concatenate two Reals to a Vector |
//! | RE | VE | VE | Prepend a Real to the left of a Vector |
//! | ST | ST | ST | Concatenate two Strings |
//! | VE | RE | VE | Append a Real to the right of a Vector |
//! | VE | VE | VE | Concatenate two Vectors |
//!
//! ## Rosy Example
//! ```text
#![doc = include_str!("test.rosy")]
//! ```
//! **Output**:
//! ```text
#![doc = include_str!("rosy_output.txt")]
//! ```
//! ## COSY INFINITY Example
//! ```text
#![doc = include_str!("test.fox")]
//! ```
//! **Output**:
//! ```text
#![doc = include_str!("cosy_output.txt")]
//! ```

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::resolve::{ExprRecipe, ScopeContext, TypeResolver, TypeSlot};
use crate::rosy_lib::RosyType;
use crate::transpile::{
    ExprFunctionCallResult, TranspilationInputContext, TranspilationOutput,
    Transpile, TranspileableExpr, ValueKind,
};
use anyhow::{Context, Error, Result};
use std::collections::BTreeSet;
use std::collections::HashSet;

/// AST node for the concatenation operator (`&`).
#[derive(Debug)]
pub struct ConcatExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl FromRule for ConcatExpr {
    fn from_rule(_pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::bail!("ConcatExpr should be created by infix parser, not FromRule")
    }
}
impl TranspileableExpr for ConcatExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        let left_type = self
            .left
            .type_of(context)
            .context("...while determining type of left side of concatenation")?;
        let right_type = self
            .right
            .type_of(context)
            .context("...while determining type of right side of concatenation")?;

        crate::rosy_lib::operators::concat::get_return_type(&left_type, &right_type)
            .ok_or(anyhow::anyhow!(
                "Cannot concatenate types '{}' and '{}' together!",
                left_type,
                right_type
            ))
    }
    fn discover_expr_function_calls(
        &self,
        resolver: &mut TypeResolver,
        ctx: &ScopeContext,
    ) -> ExprFunctionCallResult {
        if let Err(e) = resolver.discover_expr_function_calls(&self.left, ctx) {
            return ExprFunctionCallResult::HasFunctionCalls { result: Err(e) };
        }
        if let Err(e) = resolver.discover_expr_function_calls(&self.right, ctx) {
            return ExprFunctionCallResult::HasFunctionCalls { result: Err(e) };
        }
        ExprFunctionCallResult::HasFunctionCalls { result: Ok(()) }
    }
    fn build_expr_recipe(
        &self,
        resolver: &TypeResolver,
        ctx: &ScopeContext,
        deps: &mut HashSet<TypeSlot>,
    ) -> ExprRecipe {
        let left = resolver.build_expr_recipe(&self.left, ctx, deps);
        let right = resolver.build_expr_recipe(&self.right, ctx, deps);
        ExprRecipe::Concat(Box::new(left), Box::new(right))
    }
}
impl Transpile for ConcatExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        // Type check
        let _ = self
            .type_of(context)
            .map_err(|e| vec![e.context("...while verifying types of concatenation expression")])?;

        let left_output = self.left.transpile(context)
            .map_err(|errs| errs.into_iter()
                .map(|e| e.context("...while transpiling left side of concatenation"))
                .collect::<Vec<_>>())?;
        let right_output = self.right.transpile(context)
            .map_err(|errs| errs.into_iter()
                .map(|e| e.context("...while transpiling right side of concatenation"))
                .collect::<Vec<_>>())?;

        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(left_output.requested_variables.iter().cloned());
        requested_variables.extend(right_output.requested_variables.iter().cloned());

        Ok(TranspilationOutput {
            serialization: format!(
                "RosyConcat::rosy_concat({}, {})?",
                left_output.as_ref(),
                right_output.as_ref()
            ),
            requested_variables,
            value_kind: ValueKind::Owned,
        })
    }
}
