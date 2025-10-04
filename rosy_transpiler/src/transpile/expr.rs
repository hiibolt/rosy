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
                serialization: format!("&mut {n}f64"),
                requested_variables: BTreeSet::new(),
            }),
            Expr::String(s) => Ok(TranspilationOutput {
                serialization: format!("&mut \"{}\".to_string()", s.replace('"', "\\\"")),
                requested_variables: BTreeSet::new(),
            }),
            Expr::Boolean(b) => Ok(TranspilationOutput {
                serialization: format!("&mut {}", if *b { "true" } else { "false" }),
                requested_variables: BTreeSet::new(),
            }),
            Expr::Var(identifier) => {
                let TranspilationOutput {
                    serialization: serialized_identifier,
                    requested_variables
                } = identifier.transpile(context)
                    .map_err(|e| e.into_iter().map(|err| {
                        err.context(format!(
                            "...while transpiling variable identifier '{}'", identifier.name
                        ))
                    }).collect::<Vec<Error>>())?;
                
                let reference = match context.variables.get(&identifier.name)
                    .ok_or(vec!(anyhow::anyhow!("Variable '{}' is not defined in this scope!", identifier.name)))? 
                    .scope
                {
                    VariableScope::Local => "&mut ",
                    VariableScope::Arg => "",
                    VariableScope::Higher => ""
                };
                Ok(TranspilationOutput {
                    serialization: format!("{}{}", reference, serialized_identifier),
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
                let mut serialization = String::from("&mut RosyAdd::rosy_add(&*");
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
                serialization.push_str(", &*");
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
                let serialization = format!("&mut RosyST::rosy_to_string(&*{})", expr_serialization);
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
            },
            Expr::Concat(concat_expr)=> {
                // First, do a type check 
                //
                // Sneaky way to check that all terms are compatible :3
                let _ = concat_expr.type_of(context)
                    .map_err(|e| vec!(e.context("...while verifying types of concatenation expression")))?;

                let mut requested_variables = BTreeSet::new();
                let mut errors = Vec::new();
                let serialization = {
                    let mut terms = concat_expr.terms.clone();

                    // Serialize the first term as a base
                    let _ = terms.get(0)
                        .ok_or(vec!(anyhow!("Concatenation expression must have at least one term!")))?;
                    let first_term = terms.remove(0);
                    let mut serialization = match first_term.transpile(context) {
                        Ok(output) => {
                            requested_variables.extend(output.requested_variables);
                            output.serialization
                        },
                        Err(mut e) => {
                            for err in e.drain(..) {
                                errors.push(err.context("...while transpiling first term of concatenation"));
                            }
                            String::new() // dummy value to collect more errors
                        }
                    };

                    // Then, for each subsequent term, serialize and concatenate
                    for (i, term) in terms.into_iter().enumerate() {
                        serialization = format!(
                            "&mut RosyConcat::rosy_concat(&*{}, &*{})",
                            serialization,
                            match term.transpile(context) {
                                Ok(output) => {
                                    requested_variables.extend(output.requested_variables);
                                    output.serialization
                                },
                                Err(vec_err) => {
                                    for err in vec_err {
                                        errors.push(err.context(format!(
                                            "...while transpiling term {} of concatenation", i+2
                                        )));
                                    }
                                    String::new() // dummy value to collect more errors
                                }
                            }
                        );
                    }
                    
                    serialization
                };
                if errors.is_empty() {
                    Ok(TranspilationOutput {
                        serialization,
                        requested_variables
                    })
                } else {
                    Err(errors)
                }
            },
            Expr::Extract(extract_expr) => {
                // First, ensure the types are compatible
                let _ = extract_expr.type_of(context)
                    .map_err(|e| vec!(e.context("...while verifying types of extraction expression")))?;

                // Then, transpile both sides and combine
                let mut serialization = String::from("&mut RosyExtract::rosy_extract(&*");
                let mut errors = Vec::new();
                let mut requested_variables = BTreeSet::new();

                // Transpile object
                match extract_expr.object.transpile(context) {
                    Ok(output) => {
                        serialization.push_str(&output.serialization);
                        requested_variables.extend(output.requested_variables);
                    },
                    Err(mut e) => {
                        for err in e.drain(..) {
                            errors.push(err.context("...while transpiling object of extraction"));
                        }
                    }
                }

                // Transpile index
                serialization.push_str(", &*");
                match extract_expr.index.transpile(context) {
                    Ok(output) => {
                        serialization.push_str(&output.serialization);
                        requested_variables.extend(output.requested_variables);
                    },
                    Err(mut e) => {
                        for err in e.drain(..) {
                            errors.push(err.context("...while transpiling index of extraction"));
                        }
                    }
                }
                serialization.push_str(").context(\"...while trying to extract an element\")?");

                if errors.is_empty() {
                    Ok(TranspilationOutput {
                        serialization,
                        requested_variables
                    })
                } else {
                    Err(errors)
                }
            },
            Expr::Complex(complex_expr) => {
                // First, ensure the type is convertible to CM
                //
                // Sneaky way to check that the type is compatible :3
                let _ = complex_expr.type_of(context)
                    .map_err(|e| vec!(e.context("...while verifying types of complex conversion expression")))?;

                // Then, transpile the expression
                let TranspilationOutput {
                    serialization: expr_serialization,
                    requested_variables
                } = complex_expr.expr.transpile(context)
                    .map_err(|e| e.into_iter().map(|err| {
                        err.context("...while transpiling expression for CM conversion")
                    }).collect::<Vec<Error>>())?;

                // Finally, serialize the conversion
                let serialization = format!("&mut RosyCM::rosy_cm(&*{}).context(\"...while trying to convert to (CM)\")?", expr_serialization);
                Ok(TranspilationOutput {
                    serialization,
                    requested_variables
                })
            }
        }
    }
}