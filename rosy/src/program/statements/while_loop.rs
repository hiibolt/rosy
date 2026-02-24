//! WHILE loop statement implementation.
//!
//! Syntax: `WHILE <condition>; <statements> ENDWHILE;`
//!
//! The condition is evaluated before each iteration. If it evaluates to TRUE,
//! the body is executed and then the condition is checked again. When the
//! condition evaluates to FALSE, execution continues after ENDWHILE.

use std::collections::BTreeSet;
use anyhow::{Result, Context, Error, anyhow, ensure};

use crate::{
    ast::*,
    program::expressions::Expr,
    rosy_lib::RosyType,
    program::statements::Statement,
    transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TypeOf, indent}
};

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Expr,
    pub body: Vec<Statement>,
}

impl FromRule for WhileStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::while_loop, 
            "Expected `while_loop` rule when building while statement, found: {:?}", pair.as_rule());
        
        let mut inner = pair.into_inner();
        
        // Parse start_while to get condition
        let condition = {
            let mut start_while_inner = inner
                .next()
                .context("Missing first token `start_while`!")?
                .into_inner();

            let condition_pair = start_while_inner.next()
                .context("Missing condition expression in WHILE statement!")?;
            Expr::from_rule(condition_pair)
                .context("Failed to build condition expression in WHILE statement!")?
                .ok_or_else(|| anyhow::anyhow!("Expected expression for condition in WHILE statement"))?
        };

        let mut body = Vec::new();
        // Process remaining elements (statements and end)
        while let Some(element) = inner.next() {
            // Skip the end element
            if element.as_rule() == Rule::end_while {
                break;
            }

            let pair_input = element.as_str();
            if let Some(stmt) = Statement::from_rule(element)
                .with_context(|| format!("Failed to build statement from:\n{}", pair_input))? {
                body.push(stmt);
            }
        }

        Ok(Some(WhileStatement { condition, body }))
    }
}

impl Transpile for WhileStatement {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        // Verify the condition is a logical expression
        let condition_type = self.condition.type_of(context)
            .map_err(|e| vec!(e))?;
        if condition_type != RosyType::LO() {
            return Err(vec!(anyhow!(
                "WHILE condition must be of type 'LO' (logical), found '{}'", condition_type
            )));
        }

        let mut inner_context: TranspilationInputContext = context.clone();
        inner_context.in_loop = true;
        let mut requested_variables = BTreeSet::new();
        let mut serialized_statements = Vec::new();
        let mut errors = Vec::new();

        // Transpile each inner statement
        for stmt in &self.body {
            match stmt.transpile(&mut inner_context) {
                Ok(output) => {
                    serialized_statements.push(output.serialization);
                    requested_variables.extend(output.requested_variables);
                },
                Err(stmt_errors) => {
                    for e in stmt_errors {
                        errors.push(e.context("...while transpiling statement in WHILE loop"));
                    }
                }
            }
        }

        // Serialize the condition expression
        let condition_serialization = match self.condition.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(vec_err) => {
                for e in vec_err {
                    errors.push(e.context("...while transpiling condition for WHILE loop"));
                }
                return Err(errors);
            }
        };

        // Generate Rust while loop
        // Use .to_owned() to convert &mut bool to bool for the condition
        let serialization = format!(
            "while ({}).to_owned() {{\n{}\n}}",
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
