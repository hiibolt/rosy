use std::collections::BTreeSet;

use crate::ast::*;
use super::super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};


impl Transpile for ReadStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();
        
        // Serialize the identifier
        let serialized_variable_identifier = match self.identifier.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(vec_err) => {
                for err in vec_err {
                    errors.push(err.context(format!(
                        "...while transpiling identifier expression for READ into '{}'", self.identifier.name
                    )));
                }
                String::new() // dummy value to collect more errors
            }
        };

        // Get the variable type and ensure it's compatible with READ
        let variable_type = match self.identifier.type_of(context) {
            Ok(var_type) => var_type,
            Err(e) => {
                errors.push(e.context(format!(
                    "...while determining type of variable identifier for READ into '{}'", self.identifier.name
                )));
                return Err(errors); // Cannot continue without the type
            }
        };
        if !crate::rosy_lib::intrinsics::from_st::can_be_obtained(&variable_type) {
            errors.push(anyhow!(
                "Cannot READ into variable '{}' of type {}!", self.identifier.name, variable_type
            ));
            return Err(errors); // Cannot continue if the type is incompatible
        }

        // Serialize the variable type
        let serialized_variable_type = variable_type.as_rust_type();

        // Emulate the checking of the unit
        match self.unit {
            5 => {},
            _ => return Err(vec!(anyhow!(
                "Only READ from unit 5 (standard input) is supported, found unit {}!", self.unit
            ))),
        }

        // Serialize the entire function
        let serialization = format!(
            "{} = rosy_lib::intrinsics::from_st::from_stdin::<{}>().context(\"Failed to READ into {}\")?;",
            serialized_variable_identifier, serialized_variable_type, self.identifier.name
        );
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