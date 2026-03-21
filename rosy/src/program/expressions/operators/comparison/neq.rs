//! # Not-Equal Operator (`<>`)
//!
//! Tests two values for inequality. Returns `LO` (logical/boolean).
//!
//! ## Syntax
//!
//! ```text
//! expr <> expr
//! ```
//!
//! ## Type Compatibility
//!
//! | Left | Right | Result | Comment |
//! |------|-------|--------|---------|
//! | RE | RE | LO | Not-equals with epsilon tolerance |
//! | ST | ST | LO | String not-equals |
//! | LO | LO | LO | Logical not-equals |
//!
//! ## Example
//!
//! ```text
//! IF x <> 0;
//!     WRITE 6 'x is not zero';
//! ENDIF;
//! ```

use std::collections::BTreeSet;
use std::collections::HashSet;

use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::TranspileableExpr;
use crate::transpile::{Transpile, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};
use crate::rosy_lib::RosyType;

/// AST node for the not-equal operator (`<>`).
#[derive(Debug, PartialEq)]
pub struct NeqExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl FromRule for NeqExpr {
    fn from_rule(_pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        // NeqExpr is created by the infix parser, not directly from a rule
        anyhow::bail!("NeqExpr should be created by infix parser, not FromRule")
    }
}
impl TranspileableExpr for NeqExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        crate::rosy_lib::operators::neq::get_return_type(
            &self.left.type_of(context)?,
            &self.right.type_of(context)?
        ).ok_or(anyhow::anyhow!(
            "Cannot compare types '{}' and '{}' for inequality!",
            self.left.type_of(context)?,
            self.right.type_of(context)?
        ))
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::LO()))
    }
}
impl Transpile for NeqExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // First, ensure the types are compatible
        let left_type = self.left.type_of(context)
            .map_err(|e| vec!(e))?;
        let right_type = self.right.type_of(context)
            .map_err(|e| vec!(e))?;
        if crate::rosy_lib::operators::neq::get_return_type(&left_type, &right_type).is_none() {
            return Err(vec!(anyhow!(
                "Cannot compare types '{}' and '{}' for inequality!", left_type, right_type
            )));
        }

        // Then, transpile both sides and combine
        let mut errors = Vec::new();
        let mut requested_variables = BTreeSet::new();

        let left_ser = match self.left.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling left-hand side of not-equals"));
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
                    errors.push(err.context("...while transpiling right-hand side of not-equals"));
                }
                String::new()
            }
        };

        use crate::rosy_lib::RosyBaseType;
        let serialization = match (&left_type.base_type, &right_type.base_type) {
            (RosyBaseType::RE, RosyBaseType::RE) | (RosyBaseType::ST, RosyBaseType::ST)
                if left_type.dimensions == 0 && right_type.dimensions == 0
                => format!("&mut ((*{}) != (*{}))", left_ser, right_ser),
            _ => format!("&mut RosyNeq::rosy_neq(&*{}, &*{})?", left_ser, right_ser),
        };

        if errors.is_empty() {
            Ok(TranspilationOutput {
                serialization,
                requested_variables,
                ..Default::default()
            })
        } else {
            Err(errors)
        }
    }
}
