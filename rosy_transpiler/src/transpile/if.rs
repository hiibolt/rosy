use std::collections::BTreeSet;

use crate::{ast::*};
use super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput, indent};
use anyhow::{Result, Context, Error, anyhow};
use rosy_lib::RosyType;

impl Transpile for ElseIfClause {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // Verify the start, end, and step expressions are REs
        let condition_type = self.condition
            .type_of(context)
            .context("...while determining type of ELSEIF condition expression")
            .map_err(|e| vec!(e))?;
        if condition_type != RosyType::LO() {
            return Err(vec!(anyhow!("Loop start expression must be of type 'RE', found '{condition_type}'")));
        }

        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();

        // Transpile the condition
        let condition_serialization = match self.condition.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(err_vec) => {
                for err in err_vec {
                    errors.push(err.context("...while transpiling ELSEIF condition expression"));
                }
                String::new() // dummy value so we can gather all errors
            }
        };

        // Transpile the body
        let serialized_statements: Vec<String> = {
            let mut serialized_statements = Vec::new();
            let mut inner_context: TranspilationInputContext = context.clone();

            // Transpile each inner statement
            for stmt in &self.body {
                match stmt.transpile(&mut inner_context) {
                    Ok(output) => {
                        serialized_statements.push(output.serialization);
                        requested_variables.extend(output.requested_variables);
                    },
                    Err(stmt_errors) => {
                        for e in stmt_errors {
                            errors.push(e.context("...while transpiling statement in loop"));
                        }
                    }
                }
            }
            serialized_statements
        };

        let serialization = format!(
            "else if ({}).to_owned() {{\n{}\n}}",
            condition_serialization,
            indent(serialized_statements.join("\n"))
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
impl Transpile for IfStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // Verify the start, end, and step expressions are REs
        let condition_type = self.condition
            .type_of(context)
            .context("...while determining type of IF condition expression")
            .map_err(|e| vec!(e))?;
        if condition_type != RosyType::LO() {
            return Err(vec!(anyhow!("Loop start expression must be of type 'RE', found '{condition_type}'")));
        }

        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();

        // Transpile the condition
        let condition_serialization = match self.condition.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(err_vec) => {
                for err in err_vec {
                    errors.push(err.context("...while transpiling IF condition expression"));
                }
                String::new() // dummy value so we can gather all errors
            }
        };

        // Transpile the primary if clause body
        let serialized_if_statements: Vec<String> = {
            let mut serialized_if_statements = Vec::new();
            let mut inner_context: TranspilationInputContext = context.clone();

            // Transpile each inner statement
            for stmt in &self.then_body {
                match stmt.transpile(&mut inner_context) {
                    Ok(output) => {
                        serialized_if_statements.push(output.serialization);
                        requested_variables.extend(output.requested_variables);
                    },
                    Err(err_vec) => {
                        for err in err_vec {
                            errors.push(err.context("...while transpiling statement in IF body"));
                        }
                    }
                }
            }

            serialized_if_statements
        };

        // Transpile each ELSEIF clause
        let serialized_elseif_clauses = {
            let mut serialized_elseif_clauses = Vec::new();
            for elseif_clause in &self.elseif_clauses {
                match elseif_clause.transpile(context) {
                    Ok(output) => {
                        requested_variables.extend(output.requested_variables);
                        serialized_elseif_clauses.push(output.serialization);
                    },
                    Err(vec_err) => {
                        for err in vec_err {
                            errors.push(err.context("...while transpiling ELSEIF clause"));
                        }
                    }
                }
            }
            serialized_elseif_clauses
        };

        // Transpile the ELSE clause body, if it exists
        let serialized_else_clause = if let Some(else_body) = &self.else_body {
            let mut serialized_else_statements = Vec::new();
            let mut inner_context: TranspilationInputContext = context.clone();

            // Transpile each inner statement
            for stmt in else_body {
                match stmt.transpile(&mut inner_context) {
                    Ok(output) => {
                        serialized_else_statements.push(output.serialization);
                        requested_variables.extend(output.requested_variables);
                    },
                    Err(stmt_errors) => {
                        for e in stmt_errors {
                            errors.push(e.context("...while transpiling statement in loop"));
                        }
                    }
                }
            }
            format!(" else {{\n{}\n}}", indent(serialized_else_statements.join("\n")))
        } else {
            String::new()
        };

        let serialization = format!(
            "if ({}).to_owned() {{\n{}\n}}{}{}",
            condition_serialization,
            indent(serialized_if_statements.join("\n")),
            if serialized_elseif_clauses.is_empty() { 
                String::new() 
            } else { 
                format!(" {}", serialized_elseif_clauses.join(" ")) 
            },
            serialized_else_clause
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