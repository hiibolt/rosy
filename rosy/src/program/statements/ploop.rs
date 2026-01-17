use std::collections::BTreeSet;
use anyhow::{Result, Context, Error, anyhow, ensure, bail};

use crate::{
    ast::*, program::expressions::{Expr, variable_identifier::VariableIdentifier}, rosy_lib::RosyType, program::statements::Statement, transpile::{ScopedVariableData, TranspilationInputContext, TranspilationOutput, Transpile, TypeOf, VariableData, VariableScope, indent}
};

#[derive(Debug)]
pub struct PLoopStatement {
    pub iterator: String,
    pub start: Expr,
    pub end: Expr,
    pub body: Vec<Statement>,
    pub commutivityfrom_rule: Option<u8>,
    pub output: VariableIdentifier
}

impl FromRule for PLoopStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::ploop, 
            "Expected `ploop` rule when building ploop statement, found: {:?}", pair.as_rule());
        
        let mut inner = pair.into_inner();
        let (iterator, start, end) = {
            let mut start_loop_inner = inner
                .next()
                .context("Missing first token `start_loop`!")?
                .into_inner();

            let iterator = start_loop_inner.next()
                .context("Missing first token `variable_name`!")?
                .as_str().to_string();
            let start_pair = start_loop_inner.next()
                .context("Missing second token `start_expr`!")?;
            let start = Expr::from_rule(start_pair)
                .context("Failed to build `start` expression in `loop` statement!")?
                .ok_or_else(|| anyhow::anyhow!("Expected expression for `start` in `loop` statement"))?;
            let end_pair = start_loop_inner.next()
                .context("Missing third token `end_expr`!")?;
            let end = Expr::from_rule(end_pair)
                .context("Failed to build `end` expression in `loop` statement!")?
                .ok_or_else(|| anyhow::anyhow!("Expected expression for `end` in `loop` statement"))?;

            (iterator, start, end)
        };

        let mut body = Vec::new();
        // Process remaining elements (statements and end)
        let end_ploop_pair = loop {
            let element = inner.next()
                .ok_or(anyhow::anyhow!("Expected `end_ploop` statement at end of `ploop`!"))?;

            // Skip the end element
            if element.as_rule() == Rule::end_ploop {
                break element;
            }

            let pair_input = element.as_str();
            if let Some(stmt) = Statement::from_rule(element)
                .with_context(|| format!("Failed to build statement from:\n{}", pair_input))? {
                body.push(stmt);
            }
        };
        let (
            commutivityfrom_rule,
            output
        ) = {
            let mut end_ploop_inner = end_ploop_pair
                .into_inner();

            let first_pair = end_ploop_inner.next()
                .context("Missing first token in `end_ploop` statement!")?;
            let second_pair = end_ploop_inner.next()
                .context("Missing second token in `end_ploop` statement!")?;

            match (first_pair.as_rule(), second_pair.as_rule()) {
                (Rule::unit, Rule::variable_identifier) => {
                    let commutivityfrom_rule = first_pair.as_str().parse::<u8>()
                        .context("Failed to parse `commutivityfrom_rule` as u8 in `ploop` statement!")?;
                    let output = VariableIdentifier::from_rule(second_pair)
                        .context("Failed to build `output` variable identifier in `ploop` statement!")?
                        .ok_or_else(|| anyhow::anyhow!("Expected variable identifier for ploop statement"))?;
                    
                    (Some(commutivityfrom_rule), output)
                }
                (Rule::variable_identifier, Rule::semicolon) => {
                    let output = VariableIdentifier::from_rule(first_pair)
                        .context("Failed to build `output` variable identifier in `ploop` statement!")?
                        .ok_or_else(|| anyhow::anyhow!("Expected variable identifier for ploop statement"))?;
                    
                    (None, output)
                }
                _ => bail!("Expected `variable_identifier` in `end_ploop` statement!"),
            }
        };

        Ok(Some(PLoopStatement {
            iterator,
            start,
            end,
            commutivityfrom_rule,
            body,
            output
        }))
    }
}

impl Transpile for PLoopStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
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
                r#type: RosyType::RE()
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
                "let mut {} = rosy_mpi_context.get_group_num(&mut {})? + 1.0f64;",
                self.iterator,
                end_serialization
            )
        };
        let coordination_serialization = format!(
            "rosy_mpi_context.coordinate(&mut {}, {}u8, &mut {})?;",
            output_serialization,
            self.commutivityfrom_rule.unwrap_or(1),
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
