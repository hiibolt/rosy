use std::collections::HashSet;

use crate::ast::*;
use super::{Transpile, TranspilationInputContext, TranspilationOutput, LeveledVariableData, indent};
use anyhow::{Result, Error, anyhow};


impl Transpile for FunctionStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // Insert the procedure signature, but check it doesn't already exist
        if context.functions.contains_key(&self.name) ||
            matches!(context.procedures.insert(
                self.name.clone(),
                self.args.iter()
                    .map(|data| data.name.clone())
                    .collect()
            ), Some(_))
        {
            return Err(vec!(anyhow!("Procedure '{}' is already defined in this scope!", self.name)));
        }


        // Define and raise the level of any existing variables
        let mut inner_context: TranspilationInputContext = context.clone();
        let mut requested_variables = HashSet::from_iter(
            self.args.iter().map(|arg| arg.name.clone())
        );
        let mut serialized_statements = Vec::new();
        let mut errors = Vec::new();
        for arg in &self.args {
            if matches!(inner_context.variables.insert(arg.name.clone(), LeveledVariableData {
                levels_above: 0,
                data: arg.clone()
            }), Some(_)) {
                errors.push(anyhow!("Argument '{}' is already defined!", arg.name));
            }
        }
        for (_, LeveledVariableData { levels_above, .. }) in inner_context.variables.iter_mut() {
            *levels_above += 1;
        }

        // Transpile each inner statement
        for stmt in &self.body {
            match stmt.transpile(&mut inner_context) {
                Ok(output) => {
                    serialized_statements.push(output.serialization);
                    requested_variables.extend(output.requested_variables);
                },
                Err(stmt_errors) => {
                    for e in stmt_errors {
                        errors.push(e.context(format!(
                            "...while transpiling statement in procedure '{}'", self.name
                        )));
                    }
                }
            }
        }

        // Serialize arguments
        let serialized_args: Vec<String> = {
            let mut serialized_args = Vec::new();
            for var_name in requested_variables.iter() {
                let Some(var_data) = inner_context.variables
                    .get(var_name) else 
                {
                    errors.push(anyhow!(
                        "Variable '{}' was requested but not found in context!", var_name
                    ).context(format!(
                        "...while transpiling procedure '{}'", self.name
                    )));
                    continue;
                };

                serialized_args.push(format!(
                    "{}: &mut {}",
                    var_name,
                    var_data.data.r#type.as_rust_type()
                ));
            }
            /* superceded by requests? automatic optimization :3
            for var_data in self.args.iter() {
                serialized_args.push(format!(
                    "{}: &mut {}",
                    var_data.name,
                    var_data.r#type.as_rust_type()
                ));
            } */
            serialized_args
        };

        // Serialize return type
        let serialized_return_type = self.return_type.as_rust_type();

        // Serialize the entire function
        let serialization = format!(
            "fn {} ( {} ) -> {} {{\n{}\n\t{}\n}}",
            self.name, serialized_args.join(", "), 
            serialized_return_type,
            indent(serialized_statements.join("\n")),
            self.name
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