use std::collections::BTreeSet;

use crate::{ast::{FromRule, Rule}, program::statements::Statement, transpile::*};
use anyhow::{Result, Context, Error};

pub mod statements;
pub mod expressions;


#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl FromRule for Program {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Program>> {
        let mut statements = Vec::new();

        for stmt in pair.into_inner() {
            let pair_input = stmt.as_str();
            if let Some(statement) = Statement::from_rule(stmt)
                .with_context(|| format!("Failed to build statement from:\n{}", pair_input))?
            {
                statements.push(statement);
            }
        }

        Ok(Some(Program { statements }))
    }
}
impl Transpile for Program {
    fn transpile (
        &self, context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let mut serialization = Vec::new();
        let mut errors = Vec::new();
        for statement in &self.statements {
            match statement.transpile(context) {
                Ok(output) => {
                    serialization.push(output.serialization);
                },
                Err(stmt_errors) => {
                    for e in stmt_errors {
                        errors.push(e.context("...while transpiling a top-level statement"));
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(TranspilationOutput {
                serialization: serialization.join("\n"),
                requested_variables: BTreeSet::new(),
            })
        } else {
            Err(errors)
        }
    }
}