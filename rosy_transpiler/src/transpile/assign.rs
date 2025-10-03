use std::collections::HashSet;

use crate::ast::*;
use super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};


impl Transpile for AssignStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // We get the variable type and decrease its dimensions
        //  by the number of indicies
        let mut variable_type = context.variables.get(&self.name)
            .ok_or(vec!(anyhow!("Variable '{}' is not defined in this scope!", self.name)))?
            .data.r#type.clone();
        variable_type.dimensions = variable_type.dimensions
            .checked_sub(self.indicies.len())
            .ok_or(vec!(anyhow!(
                "Variable '{}' does not have enough dimensions to index into it (tried to index {} times, but it only has {} dimensions)!",
                self.name, self.indicies.len(), variable_type.dimensions
            )))?;

        // Then, ensure the value type is compatible
        let value_type = self.value.type_of(context)
            .map_err(|e| vec!(
                e.context("...while determining type of value expression for assignment")
            ))?;
        if variable_type != value_type {
            return Err(vec!(anyhow!(
                "Cannot assign value of type '{}' to variable '{}' of type '{}'!", 
                value_type, self.name, variable_type
            )));
        }
        
        // Serialize the indicies
        let mut serialized_indicies = String::new();
        let mut requested_variables = HashSet::new();
        let mut errors = Vec::new();
        for dim in &self.indicies {
            match dim.transpile(context) {
                Ok(output) => {
                    serialized_indicies.push_str(&format!("[({}).to_owned() as usize]", output.serialization));
                    requested_variables.extend(output.requested_variables);
                },
                Err(dim_errors) => {
                    for err in dim_errors {
                        errors.push(err.context(format!(
                            "...while transpiling index expression for assignment to '{}'", self.name
                        )));
                    }
                }
            }
        }

        // Serialize the value
        let serialized_value = match self.value.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(value_errors) => {
                for err in value_errors {
                    errors.push(err.context(format!(
                        "...while transpiling value expression for assignment to '{}'", self.name
                    )));
                }
                String::new()
            }
        };

        // Serialize the entire function
        let serialization = format!(
            "{}{} = ({}).to_owned();",
            self.name, serialized_indicies, serialized_value
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