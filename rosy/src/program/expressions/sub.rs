use std::collections::BTreeSet;

use crate::ast::{FromRule, Rule};
use crate::transpile::TranspileWithType;
use crate::program::expressions::Expr;
use crate::rosy_lib::RosyType;
use crate::transpile::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};

#[derive(Debug, PartialEq)]
pub struct SubExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl FromRule for SubExpr {
    fn from_rule(_pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        // SubExpr is created by the infix parser, not directly from a rule
        anyhow::bail!("SubExpr should be created by infix parser, not FromRule")
    }
}
impl TranspileWithType for SubExpr {}
impl TypeOf for SubExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        crate::rosy_lib::operators::sub::get_return_type(
            &self.left.type_of(context)?,
            &self.right.type_of(context)?
        ).ok_or(anyhow::anyhow!(
            "Cannot subtract types '{}' and '{}' together!",
            self.left.type_of(context)?,
            self.right.type_of(context)?
        ))
    }
}
impl Transpile for SubExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // First, ensure the types are compatible
        let left_type = self.left.type_of(context)
            .map_err(|e| vec!(e))?;
        let right_type = self.right.type_of(context)
            .map_err(|e| vec!(e))?;
        if crate::rosy_lib::operators::sub::get_return_type(&left_type, &right_type).is_none() {
            return Err(vec!(anyhow!(
                "Cannot subtract types '{}' and '{}' together!", left_type, right_type
            )));
        }

        // Then, transpile both sides and combine
        let mut serialization = String::from("&mut RosySub::rosy_sub(&*");
        let mut errors = Vec::new();
        let mut requested_variables = BTreeSet::new();

        // Transpile left
        match self.left.transpile(context) {
            Ok(output) => {
                serialization.push_str(&output.serialization);
                requested_variables.extend(output.requested_variables);
            },
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling left-hand side of subtraction"));
                }
            }
        }

        // Transpile right
        serialization.push_str(", &*");
        match self.right.transpile(context) {
            Ok(output) => {
                serialization.push_str(&output.serialization);
                requested_variables.extend(output.requested_variables);
            },
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling right-hand side of subtraction"));
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
