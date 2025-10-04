use std::collections::BTreeSet;

use crate::ast::*;
use super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput, VariableScope};
use anyhow::{Result, Error, anyhow};

impl Transpile for AssignStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // Get the variable type and ensure the value type is compatible
        let variable_type = self.identifier.type_of(context)
            .map_err(|e| vec!(
                e.context("...while determining type of variable identifier for assignment")
            ))?;
        let value_type = self.value.type_of(context)
            .map_err(|e| vec!(
                e.context("...while determining type of value expression for assignment")
            ))?;
        if variable_type != value_type {
            return Err(vec!(anyhow!(
                "Cannot assign value of type '{}' to variable '{}' of type '{}'!", 
                value_type, self.identifier.name, variable_type
            )));
        }

        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();
        
        // Serialize the identifier
        let serialized_identifier = match self.identifier.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(vec_err) => {
                for err in vec_err {
                    errors.push(err.context(format!(
                        "...while transpiling identifier expression for assigment to '{}'", self.identifier.name
                    )));
                }
                String::new() // dummy value to collect more errors
            }
        };

        // Serialize the value
        let serialized_value = match self.value.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(value_errors) => {
                for err in value_errors {
                    errors.push(err.context(format!(
                        "...while transpiling value expression for assignment to '{}'", self.identifier.name
                    )));
                }
                String::new() // dummy value to collect more errors
            }
        };

        // Serialize the entire function
        let dereference = match context.variables.get(&self.identifier.name)
            .ok_or(vec!(anyhow::anyhow!("Variable '{}' is not defined in this scope!", self.identifier.name)))? 
            .scope
        {
            VariableScope::Local => "",
            VariableScope::Arg => "*",
            VariableScope::Higher => {
                // Also add to requested variables
                requested_variables.insert(self.identifier.name.clone());
                "*"
            }
        };
        let serialization = format!(
            "{}{} = ({}).to_owned();",
            dereference, serialized_identifier, serialized_value
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