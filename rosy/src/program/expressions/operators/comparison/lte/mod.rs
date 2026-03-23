//! # Less-Than-or-Equal Operator (`<=`)
//!
//! Numeric or lexicographic less-than-or-equal comparison. Returns `LO`.
//!
//! ## Syntax
//!
//! ```text
//! expr <= expr
//! ```
//!
//! ## Type Compatibility
//!
//! | Left | Right | Result | Comment |
//! |------|-------|--------|---------|
//! | RE | RE | LO | Numeric less-than-or-equal |
//! | ST | ST | LO | Lexicographic ordering |
//!
//! ## Rosy Example
#![doc = include_str!("test.rosy")]
//! **Output**:
#![doc = include_str!("rosy_output.txt")]
//! ## COSY Example
#![doc = include_str!("test.fox")]
//! **Output**:
#![doc = include_str!("cosy_output.txt")]

use std::collections::BTreeSet;
use std::collections::HashSet;

use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::TranspileableExpr;
use crate::transpile::{Transpile, TranspilationInputContext, TranspilationOutput, ValueKind};
use anyhow::{Result, Error, anyhow};
use crate::rosy_lib::RosyType;

/// AST node for the less-than-or-equal operator (`<=`).
#[derive(Debug, PartialEq)]
pub struct LteExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl FromRule for LteExpr {
    fn from_rule(_pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::bail!("LteExpr should be created by infix parser, not FromRule")
    }
}
impl TranspileableExpr for LteExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        crate::rosy_lib::operators::lte::get_return_type(
            &self.left.type_of(context)?,
            &self.right.type_of(context)?
        ).ok_or(anyhow::anyhow!(
            "Cannot compare types '{}' and '{}' with less-than-or-equal!",
            self.left.type_of(context)?,
            self.right.type_of(context)?
        ))
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::LO()))
    }
}
impl Transpile for LteExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        let left_type = self.left.type_of(context)
            .map_err(|e| vec!(e))?;
        let right_type = self.right.type_of(context)
            .map_err(|e| vec!(e))?;
        if crate::rosy_lib::operators::lte::get_return_type(&left_type, &right_type).is_none() {
            return Err(vec!(anyhow!(
                "Cannot compare types '{}' and '{}' with less-than-or-equal!", left_type, right_type
            )));
        }

        let mut errors = Vec::new();
        let mut requested_variables = BTreeSet::new();

        let left_output = match self.left.transpile(context) {
            Ok(output) => output,
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling left-hand side of less-than-or-equal"));
                }
                TranspilationOutput::default()
            }
        };
        requested_variables.extend(left_output.requested_variables.iter().cloned());

        let right_output = match self.right.transpile(context) {
            Ok(output) => output,
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling right-hand side of less-than-or-equal"));
                }
                TranspilationOutput::default()
            }
        };
        requested_variables.extend(right_output.requested_variables.iter().cloned());

        use crate::rosy_lib::RosyBaseType;
        let serialization = match (&left_type.base_type, &right_type.base_type) {
            (RosyBaseType::RE, RosyBaseType::RE) | (RosyBaseType::ST, RosyBaseType::ST)
                if left_type.dimensions == 0 && right_type.dimensions == 0
                => format!("({} <= {})", left_output.as_value(), right_output.as_value()),
            _ => format!("RosyLte::rosy_lte({}, {})?", left_output.as_ref(), right_output.as_ref()),
        };

        if errors.is_empty() {
            Ok(TranspilationOutput {
                serialization,
                requested_variables,
                value_kind: ValueKind::Owned,
            })
        } else {
            Err(errors)
        }
    }
}
