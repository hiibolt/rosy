use crate::ast::*;
use super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput, VariableScope};
use std::collections::BTreeSet;
use anyhow::{Result, Error, anyhow};

impl Transpile for Expr {
    fn transpile (
        &self, context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        match self {
            Expr::Number(n) => Ok(TranspilationOutput {
                serialization: format!("&{n}f64"),
                requested_variables: BTreeSet::new(),
            }),
            Expr::String(s) => Ok(TranspilationOutput {
                serialization: format!("&\"{}\".to_string()", s.replace('"', "\\\"")),
                requested_variables: BTreeSet::new(),
            }),
            Expr::Boolean(b) => Ok(TranspilationOutput {
                serialization: format!("&{}", if *b { "true" } else { "false" }),
                requested_variables: BTreeSet::new(),
            }),
            Expr::Var(name) => {
                let scoped_var_data = context.variables.get(name)
                    .ok_or(vec!(anyhow!("Variable '{}' is not defined in this scope!", name)))?;

                let mut serialization = name.clone();
                let mut requested_variables = BTreeSet::new();
                match scoped_var_data.scope {
                    VariableScope::Higher => {
                        serialization = format!("(&*{serialization})");
                        requested_variables.insert(name.clone());
                    },
                    VariableScope::Arg => {
                        serialization = format!("(&*{serialization})");
                    },
                    VariableScope::Local => {
                        serialization = format!("&{serialization}");
                    }
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
                let mut requested_variables = BTreeSet::new();

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
            },
            Expr::StringConvert { expr } => {
                // First, ensure the type is convertible to ST
                let expr_type = expr.type_of(context)
                    .map_err(|e| vec!(e))?;
                if rosy_lib::intrinsics::st::get_return_type(&expr_type).is_none() {
                    return Err(vec!(anyhow!(
                        "Cannot convert type '{}' to 'ST'!", expr_type
                    )));
                }

                // Then, transpile the expression
                let TranspilationOutput {
                    serialization: expr_serialization,
                    requested_variables
                } = expr.transpile(context)
                    .map_err(|e| e.into_iter().map(|err| {
                        err.context("...while transpiling expression for STRING conversion")
                    }).collect::<Vec<Error>>())?;

                // Finally, serialize the conversion
                let serialization = format!("({}).rosy_to_string()", expr_serialization);
                Ok(TranspilationOutput {
                    serialization,
                    requested_variables
                })
            },
            Expr::FunctionCall { name, args } => {
                let function_call_statement = FunctionCallStatement {
                    name: name.clone(),
                    args: args.clone()
                };
                let mut output = function_call_statement
                    .transpile(context)
                    .map_err(|e| e.into_iter().map(|err| {
                        err.context(format!("...while transpiling function call to '{}'", name))
                    }).collect::<Vec<Error>>())?;
                output.serialization = format!("&{}", output.serialization);
                Ok(output)
            }
            _ => todo!()
        }
    }
}