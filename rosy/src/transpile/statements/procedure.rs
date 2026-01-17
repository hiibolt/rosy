use std::collections::BTreeSet;

use crate::{ast::*};
use super::super::{Transpile, TranspilationInputContext, VariableData, TranspilationInputProcedureContext, TranspilationOutput, ScopedVariableData, VariableScope, indent};
use anyhow::{Result, Error, anyhow};


impl Transpile for ProcedureStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // Insert the procedure signature, but check it doesn't already exist
        if context.functions.contains_key(&self.name) || 
            matches!(context.procedures.insert(
                self.name.clone(),
                TranspilationInputProcedureContext {
                    args: self.args.iter()
                        .map(|arg| VariableData {
                            name: arg.name.clone(),
                            r#type: arg.r#type.clone(),
                            total_dimensions: arg.dimension_exprs.len(),
                        })
                        .collect(),
                    requested_variables: BTreeSet::new()
                }
            ), Some(_))
        {
            return Err(vec!(anyhow!("Procedure '{}' is already defined in this scope!", self.name)));
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
                data: VariableData {
                    name: arg.name.clone(),
                    r#type: arg.r#type.clone(),
                    total_dimensions: arg.dimension_exprs.len(),
                }
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
                            "...while transpiling statement in procedure '{}'", self.name
                        )));
                    }
                }
            }
        }

        // Update the procedure context with the requested variables,
        //  first removing those which are locally defined or args
        requested_variables = requested_variables.into_iter()
            .filter(|var| {
                if let Some(var_data) = inner_context.variables.get(var) {
                    !matches!(var_data.scope, VariableScope::Local | VariableScope::Arg)
                } else {
                    true
                }
            })
            .collect();
        if let Some(proc_context) = context.procedures.get_mut(&self.name) {
            proc_context.requested_variables = requested_variables.clone();
        } else {
            errors.push(anyhow!(
                "Procedure '{}' was not found in context after being inserted!", self.name
            ).context("...while updating procedure context"));
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
            for var_data in self.args.iter() {
                serialized_args.push(format!(
                    "{}: &mut {}",
                    var_data.name,
                    var_data.r#type.as_rust_type()
                ));
            }
            serialized_args
        };

        let serialization = format!(
            "fn {} ( {} ) -> Result<()> {{\n{}\n\n\tOk(())\n}}",
            self.name, serialized_args.join(", "), indent(serialized_statements.join("\n"))
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