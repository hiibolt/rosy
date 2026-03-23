//! # Logical NOT Operator
//!
//! Inverts a logical (boolean) value.
//!
//! ## Syntax
//!
//! ```text
//! NOT expr
//! ```
//!
//! ## Supported Types
//!
//! | Input | Result |
//! |-------|--------|
//! | LO | LO |
//!
//! ## Example
//!
//! ```text
//! VARIABLE (LO) flag;
//! flag := TRUE;
//! IF NOT flag;
//!     WRITE 6 'flag is false';
//! ENDIF;
//! ```
//!
//! ```rosy_test_raw
//! --- rosy ---
//! BEGIN;
//!     VARIABLE (LO) B;
//!     B := !TRUE;
//!     WRITE 6 B;
//!     B := !FALSE;
//!     WRITE 6 B;
//! END;
//! --- fox ---
//! BEGIN;
//! PROCEDURE RUN;
//!     VARIABLE B 1;
//!     B := NOT TRUE;
//!     WRITE 6 B;
//!     B := NOT FALSE;
//!     WRITE 6 B;
//! ENDPROCEDURE;
//! RUN;
//! END;
//! ```

use std::collections::BTreeSet;
use std::collections::HashSet;

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::TranspileableExpr;
use crate::transpile::{Transpile, TranspilationInputContext, TranspilationOutput, ValueKind};
use anyhow::{Result, Error, anyhow};
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// Logical NOT expression (unary operator).
/// Supports both `!x` and `NOT x` syntax.
#[derive(Debug, PartialEq)]
pub struct NotExpr {
    pub operand: Box<Expr>,
}

impl FromRule for NotExpr {
    fn from_rule(_pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::bail!("NotExpr should be created by prefix parser, not FromRule")
    }
}
impl TranspileableExpr for NotExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        crate::rosy_lib::operators::not::get_return_type(
            &self.operand.type_of(context)?
        ).ok_or(anyhow::anyhow!(
            "Cannot apply NOT to type '{}'!",
            self.operand.type_of(context)?
        ))
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::LO()))
    }
}
impl Transpile for NotExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        let operand_type = self.operand.type_of(context)
            .map_err(|e| vec!(e))?;
        if crate::rosy_lib::operators::not::get_return_type(&operand_type).is_none() {
            return Err(vec!(anyhow!(
                "Cannot apply NOT to type '{}'!", operand_type
            )));
        }

        let mut errors = Vec::new();
        let mut requested_variables = BTreeSet::new();

        let operand_output = match self.operand.transpile(context) {
            Ok(output) => output,
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling operand of NOT"));
                }
                TranspilationOutput::default()
            }
        };
        requested_variables.extend(operand_output.requested_variables.iter().cloned());

        let serialization = format!("RosyNot::rosy_not({})?", operand_output.as_ref());

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
