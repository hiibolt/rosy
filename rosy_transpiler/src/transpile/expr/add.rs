use std::collections::BTreeSet;

use crate::ast::*;
use super::super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};
use rosy_lib::RosyType;

impl TypeOf for AddExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        rosy_lib::operators::add::get_return_type(
            &self.left.type_of(context)?,
            &self.right.type_of(context)?
        ).ok_or(anyhow::anyhow!(
            "Cannot add types '{}' and '{}' together!",
            self.left.type_of(context)?,
            self.right.type_of(context)?
        ))
    }
}
impl Transpile for AddExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // First, ensure the types are compatible
        let left_type = self.left.type_of(context)
            .map_err(|e| vec!(e))?;
        let right_type = self.right.type_of(context)
            .map_err(|e| vec!(e))?;
        if rosy_lib::operators::add::get_return_type(&left_type, &right_type).is_none() {
            return Err(vec!(anyhow!(
                "Cannot add types '{}' and '{}' together!", left_type, right_type
            )));
        }

        // Then, transpile both sides and combine
        let mut serialization = String::from("&mut RosyAdd::rosy_add(&*");
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
                    errors.push(err.context("...while transpiling left-hand side of addition"));
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
                    errors.push(err.context("...while transpiling right-hand side of addition"));
                }
            }
        }
        serialization.push(')');

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