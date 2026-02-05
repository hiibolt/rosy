use std::collections::BTreeSet;

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::TranspileWithType;
use crate::transpile::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};
use crate::rosy_lib::RosyType;

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
impl TranspileWithType for NotExpr {}
impl TypeOf for NotExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        crate::rosy_lib::operators::not::get_return_type(
            &self.operand.type_of(context)?
        ).ok_or(anyhow::anyhow!(
            "Cannot apply NOT to type '{}'!",
            self.operand.type_of(context)?
        ))
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

        let mut serialization = String::from("&mut RosyNot::rosy_not(&*");
        let mut errors = Vec::new();
        let mut requested_variables = BTreeSet::new();

        match self.operand.transpile(context) {
            Ok(output) => {
                serialization.push_str(&output.serialization);
                requested_variables.extend(output.requested_variables);
            },
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling operand of NOT"));
                }
            }
        }
        serialization.push_str(")?");

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
