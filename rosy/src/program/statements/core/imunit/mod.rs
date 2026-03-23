//! # IMUNIT Statement
//!
//! Returns the imaginary unit *i* as a CM (Complex64) value.
//!
//! ## Syntax
//!
//! ```text
//! IMUNIT v;
//! ```
//!
//! ## Rosy Example
#![doc = include_str!("test.rosy")]
//! **Output**:
#![doc = include_str!("rosy_output.txt")]
//! ## COSY Example
#![doc = include_str!("test.fox")]
//! **Output**:
#![doc = include_str!("cosy_output.txt")]

use std::collections::BTreeSet;
use anyhow::{Result, Context, Error, ensure};

use crate::{
    ast::*,
    program::expressions::core::variable_identifier::VariableIdentifier,
    transpile::{
        TranspilationInputContext, TranspilationOutput, Transpile,
        TranspileableStatement, VariableScope, add_context_to_all,
    },
};

#[derive(Debug)]
pub struct ImunitStatement {
    pub identifier: VariableIdentifier,
}

impl FromRule for ImunitStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(
            pair.as_rule() == Rule::imunit,
            "Expected `imunit` rule when building IMUNIT statement, found: {:?}",
            pair.as_rule()
        );

        let mut inner = pair.into_inner();

        let expr_pair = inner.next()
            .context("Missing variable expression in IMUNIT!")?;
        let identifier = VariableIdentifier::from_rule(expr_pair)
            .context("Failed to build variable identifier in IMUNIT")?
            .ok_or_else(|| anyhow::anyhow!("Expected variable identifier in IMUNIT"))?;

        Ok(Some(ImunitStatement { identifier }))
    }
}

impl TranspileableStatement for ImunitStatement {}

impl Transpile for ImunitStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        let mut requested_variables = BTreeSet::new();

        let output = self.identifier.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling identifier in IMUNIT".to_string()))?;
        requested_variables.extend(output.requested_variables.clone());

        let dereference = match context.variables.get(&self.identifier.name)
            .ok_or_else(|| vec![anyhow::anyhow!("Variable '{}' is not defined in this scope!", self.identifier.name)])?
            .scope
        {
            VariableScope::Local => "",
            VariableScope::Arg => "*",
            VariableScope::Higher => {
                requested_variables.insert(self.identifier.name.clone());
                "*"
            }
        };

        // Imaginary unit: Complex64::new(0.0, 1.0) = i
        let serialization = format!(
            "{}{} = num_complex::Complex64::new(0.0, 1.0);",
            dereference, output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
            ..Default::default()
        })
    }
}
