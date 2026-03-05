use std::collections::BTreeSet;

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::TranspileWithType;
use crate::transpile::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};
use crate::rosy_lib::RosyType;

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
impl TranspileWithType for LteExpr {}
impl TypeOf for LteExpr {
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
}
impl Transpile for LteExpr {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
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

        let mut serialization = String::from("&mut RosyLte::rosy_lte(&*");
        let mut errors = Vec::new();
        let mut requested_variables = BTreeSet::new();

        match self.left.transpile(context) {
            Ok(output) => {
                serialization.push_str(&output.serialization);
                requested_variables.extend(output.requested_variables);
            },
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling left-hand side of less-than-or-equal"));
                }
            }
        }

        serialization.push_str(", &*");
        match self.right.transpile(context) {
            Ok(output) => {
                serialization.push_str(&output.serialization);
                requested_variables.extend(output.requested_variables);
            },
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling right-hand side of less-than-or-equal"));
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
