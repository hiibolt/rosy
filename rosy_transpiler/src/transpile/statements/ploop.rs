use std::collections::BTreeSet;

use crate::{ast::*};
use super::super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput, ScopedVariableData, VariableScope, indent};
use anyhow::{Result, Error, anyhow};
use rosy_lib::RosyType;


impl Transpile for PLoopStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // Verify the start and end expressions are REs
        let start_type = self.start.type_of(context)
            .map_err(|e| vec!(e))?;
        if start_type != RosyType::RE() {
            return Err(vec!(anyhow!(
                "Loop start expression must be of type 'RE', found '{}'", start_type
            )));
        }
        let end_type = self.end.type_of(context)
            .map_err(|e| vec!(e))?;
        if end_type != RosyType::RE() {
            return Err(vec!(anyhow!(
                "Loop end expression must be of type 'RE', found '{}'", end_type
            )));
        }

        // Define the iterator
        let mut inner_context = context.clone();
        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();
        if matches!(inner_context.variables.insert(self.iterator.clone(), ScopedVariableData { 
            scope: VariableScope::Local,
            data: VariableData { 
                name: self.iterator.clone(),
                r#type: RosyType::RE(),
                dimension_exprs: vec![]
            }
        }), Some(_)) {
            return Err(vec!(anyhow!(
                "Iterator variable '{}' is already defined in this scope!", self.iterator
            )));
        }

        // Transpile each inner statement
        let mut serialized_statements = Vec::new();
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

        // Serialize the start and end expressions
        let _start_serialization = match self.start.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(vec_err) => {
                for e in vec_err {
                    errors.push(e.context(format!(
                        "...while transpiling start expression for loop with iterator '{}'",
                        self.iterator
                    )));
                }
                return Err(errors);
            }
        };
        let end_serialization = match self.end.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(vec_err) => {
                for e in vec_err {
                    errors.push(e.context(format!(
                        "...while transpiling end expression for loop with iterator '{}'",
                        self.iterator
                    )));
                }
                return Err(errors);
            }
        };

        // Check the type of the output array
        let output_type = self.output.type_of(context)
            .map_err(|e| vec!(e))?;
        if output_type.dimensions < 1 && output_type != RosyType::VE() {
            return Err(vec!(anyhow!(
                "Output variable '{}' for a PLOOP must be an array type, found '{}'", self.output.name, output_type
            )));
        }

        // Serialize the output identifier
        let output_serialization = match self.output.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(vec_err) => {
                for e in vec_err {
                    errors.push(e.context(format!(
                        "...while transpiling output variable identifier '{}'",
                        self.output.name
                    )));
                }
                return Err(errors);
            }
        };


        let iterator_declaration_serialization = {
            requested_variables.insert("rosy_mpi_context".to_string());
            format!(
                "let mut {} = rosy_mpi_context.get_rank(&mut {})? + 1.0f64;",
                self.iterator,
                end_serialization
            )
        };
        let coordination_serialization = format!(
            "rosy_mpi_context.coordinate(&mut {}, {}u8, &mut {})?;",
            output_serialization,
            self.commutivity_rule.unwrap_or(1),
            end_serialization
        );
        let serialization = format!(
            "{{\n\t{}\n\n{}\n\n\t{}\n}}",
            iterator_declaration_serialization,
            indent(serialized_statements.join("\n")),
            coordination_serialization
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