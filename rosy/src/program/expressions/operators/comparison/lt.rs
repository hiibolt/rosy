//! # Less-Than Operator (`<`)
//!
//! Numeric or lexicographic less-than comparison. Returns `LO`.
//!
//! ## Syntax
//!
//! ```text
//! expr < expr
//! ```
//!
//! ## Type Compatibility
//!
//! | Left | Right | Result | Comment |
//! |------|-------|--------|---------|
//! | RE | RE | LO | Numeric less-than |
//! | ST | ST | LO | Lexicographic ordering |

use std::collections::BTreeSet;
use std::collections::HashSet;

use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::TranspileableExpr;
use crate::transpile::{Transpile, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};
use crate::rosy_lib::RosyType;

/// AST node for the less-than operator (`<`).
#[derive(Debug, PartialEq)]
pub struct LtExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl FromRule for LtExpr {
    fn from_rule(_pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::bail!("LtExpr should be created by infix parser, not FromRule")
    }
}
impl TranspileableExpr for LtExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        crate::rosy_lib::operators::lt::get_return_type(
            &self.left.type_of(context)?,
            &self.right.type_of(context)?
        ).ok_or(anyhow::anyhow!(
            "Cannot compare types '{}' and '{}' with less-than!",
            self.left.type_of(context)?,
            self.right.type_of(context)?
        ))
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::LO()))
    }
}
impl Transpile for LtExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        let left_type = self.left.type_of(context)
            .map_err(|e| vec!(e))?;
        let right_type = self.right.type_of(context)
            .map_err(|e| vec!(e))?;
        if crate::rosy_lib::operators::lt::get_return_type(&left_type, &right_type).is_none() {
            return Err(vec!(anyhow!(
                "Cannot compare types '{}' and '{}' with less-than!", left_type, right_type
            )));
        }

        let mut errors = Vec::new();
        let mut requested_variables = BTreeSet::new();

        let left_ser = match self.left.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling left-hand side of less-than"));
                }
                String::new()
            }
        };

        let right_ser = match self.right.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling right-hand side of less-than"));
                }
                String::new()
            }
        };

        use crate::rosy_lib::RosyBaseType;
        let serialization = match (&left_type.base_type, &right_type.base_type) {
            (RosyBaseType::RE, RosyBaseType::RE) | (RosyBaseType::ST, RosyBaseType::ST)
                if left_type.dimensions == 0 && right_type.dimensions == 0
                => format!("&mut ((*{}) < (*{}))", left_ser, right_ser),
            _ => format!("&mut RosyLt::rosy_lt(&*{}, &*{})?", left_ser, right_ser),
        };

        if errors.is_empty() {
            Ok(TranspilationOutput {
                serialization,
                requested_variables
            })
        } else {
            Err(errors)
        }
    }
}
