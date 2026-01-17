use std::collections::BTreeSet;

use crate::ast::*;
use super::super::{Transpile, TranspilationInputContext, TypeOf, TranspilationOutput, VariableScope};
use anyhow::{Result, Error, anyhow};

pub fn function_call_transpile_helper (
    name: &String,
    args: &Vec<Expr>,
    context: &mut TranspilationInputContext
) -> Result<TranspilationOutput, Vec<Error>> {
    // Start by checking that the function exists
    let func_context = match context.functions.get(name) {
        Some(ctx) => ctx,
        None => return Err(vec!(anyhow!("Function '{}' is not defined in this scope, can't transpile function call!", name)))
    }.clone();

    // Check that the number of arguments is correct
    if func_context.args.len() != args.len() {
        return Err(vec!(anyhow!(
            "Function '{}' expects {} arguments, but {} were provided!",
            name, func_context.args.len(), args.len()
        )));
    }
    let mut errors = Vec::new();
    let mut requested_variables = BTreeSet::new();
    let mut serialized_args = Vec::new();
    // Serialize the requested variables from the function context
    for var in &func_context.requested_variables {
        let var_data = context.variables.get(var)
            .ok_or(vec!(anyhow!(
                "Could not find variable '{}' requested by function '{}'",
                var, name
            )))?;
        
        let serialized_arg = match var_data.scope {
            VariableScope::Higher => format!("{}", var),
            VariableScope::Arg => format!("{}", var),
            VariableScope::Local => format!("&mut {}", var)
        };
        serialized_args.push(serialized_arg);
    }

    // Add the manual arguments
    for (i, arg_expr) in args.iter().enumerate() {
        match arg_expr.transpile(context) {
            Ok(arg_output) => {
                // Check the type is correct
                let provided_type = arg_expr.type_of(context)
                    .map_err(|e| vec!(e))?;
                let expected_type = func_context
                    .args
                    .get(i)
                    .ok_or(vec!(anyhow!(
                        "Function '{}' expects {} arguments, but {} were provided!",
                        name, func_context.args.len(), args.len()
                    )))?
                    .r#type
                    .clone();
                if provided_type != expected_type {
                    errors.push(anyhow!(
                        "Function '{}' expects argument {} ('{}') to be of type '{}', but type '{}' was provided!",
                        name, i+1, func_context.args[i].name, expected_type, provided_type
                    ));
                } else {
                    // If the type is correct, add the serialization
                    serialized_args.push(arg_output.serialization);
                    requested_variables.extend(arg_output.requested_variables);
                }
            },
            Err(arg_errors) => {
                for e in arg_errors {
                    errors.push(e.context(format!(
                        "...while transpiling argument {} for function '{}'", i+1, name
                    )));
                }
            }
        }
    }

    // Serialize the entire function
    let serialization = format!(
        "{}({}).context(\"...while calling function '{}'\")?",
        name, serialized_args.join(", "), name
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