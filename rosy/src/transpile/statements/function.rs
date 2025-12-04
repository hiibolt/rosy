use std::collections::BTreeSet;

use crate::ast::*;
use super::super::{Transpile, TranspilationInputContext, TranspilationOutput, ScopedVariableData, VariableScope, TranspilationInputFunctionContext, indent};
use anyhow::{Result, Error, anyhow};


impl Transpile for FunctionStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // Insert the function signature, but check it doesn't already exist
        if context.functions.contains_key(&self.name) ||
            matches!(context.functions.insert(
                self.name.clone(),
                TranspilationInputFunctionContext {
                    return_type: self.return_type.clone(),
                    args: self.args.clone(),
                    requested_variables: BTreeSet::new()
                }
            ), Some(_))
        {
            return Err(vec!(anyhow!("Function '{}' is already defined in this scope!", self.name)));
        }


        // Define and raise the level of any existing variables
        let mut inner_context: TranspilationInputContext = context.clone();
        let mut requested_variables = BTreeSet::new();
        let mut serialized_statements = Vec::new();
        let mut errors = Vec::new();
        for (_, ScopedVariableData { scope, .. }) in inner_context.variables.iter_mut() {
            *scope = match *scope {
                VariableScope::Local => VariableScope::Higher,
                VariableScope::Arg => VariableScope::Higher,
                VariableScope::Higher => VariableScope::Higher
            }
        }
        for arg in &self.args {
            if matches!(inner_context.variables.insert(arg.name.clone(), ScopedVariableData {
                scope: VariableScope::Arg,
                data: arg.clone()
            }), Some(_)) {
                errors.push(anyhow!("Argument '{}' is already defined!", arg.name));
            }
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
                            "...while transpiling statement in function '{}'", self.name
                        )));
                    }
                }
            }
        }

        // Update the function context with the requested variables
        if let Some(func_context) = context.functions.get_mut(&self.name) {
            func_context.requested_variables = requested_variables.clone();
        } else {
            errors.push(anyhow!(
                "Function '{}' was not found in context after being inserted!", self.name
            ).context("...while updating function context"));
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
                        "...while transpiling function '{}'", self.name
                    )));
                    continue;
                };

                serialized_args.push(format!(
                    "{}: &mut {}",
                    var_name,
                    var_data.data.r#type.as_rust_type()
                ));
            }
            for var_data in self.args.iter() {
                serialized_args.push(format!(
                    "{}: &{}",
                    var_data.name,
                    var_data.r#type.as_rust_type()
                ));
            }
            serialized_args
        };

        // Serialize return type
        let serialized_return_type = self.return_type.as_rust_type();

        // Serialize the entire function
        let serialization = format!(
            "fn {} ( {} ) -> Result<{}> {{\n{}\n\t{}\n}}",
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