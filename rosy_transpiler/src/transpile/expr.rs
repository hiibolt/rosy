use crate::ast::*;
use super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use std::collections::HashSet;
use anyhow::{Result, Error, anyhow};

impl Transpile for Expr {
    fn transpile (
        &self, context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        match self {
            Expr::Number(n) => Ok(TranspilationOutput {
                serialization: format!("&{n}f64"),
                requested_variables: HashSet::new(),
            }),
            Expr::Var(name) => {
                let leveled_var_data = context.variables.get(name)
                    .ok_or(vec!(anyhow!("Variable '{}' is not defined in this scope!", name)))?;

                let mut serialization = name.clone();
                let mut requested_variables = HashSet::new();
                if leveled_var_data.levels_above > 0 {
                    serialization = format!("(&*{serialization})");
                    requested_variables.insert(name.clone());
                } else {
                    serialization = format!("&{serialization}");
                }
                
                Ok(TranspilationOutput {
                    serialization,
                    requested_variables
                })
            },
            Expr::Add { left, right } => {
                // First, ensure the types are compatible
                let left_type = left.type_of(context)
                    .map_err(|e| vec!(e))?;
                let right_type = right.type_of(context)
                    .map_err(|e| vec!(e))?;
                if rosy_lib::operators::add::get_return_type(&left_type, &right_type).is_none() {
                    return Err(vec!(anyhow!(
                        "Cannot add types '{}' and '{}' together!", left_type, right_type
                    )));
                }

                // Then, transpile both sides and combine
                let mut serialization = String::new();
                let mut errors = Vec::new();
                let mut requested_variables = HashSet::new();

                // Transpile left
                match left.transpile(context) {
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
                serialization.push_str(".rosy_add(");
                match right.transpile(context) {
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
            _ => todo!()
        }
    }
}