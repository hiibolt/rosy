//! BREAK statement implementation.
//!
//! Syntax: `BREAK;`
//!
//! Exits the innermost enclosing WHILE or LOOP block immediately.
//! Only valid inside WHILE or LOOP contexts - not valid inside PLOOP,
//! and PROCEDURE/FUNCTION definitions create scope boundaries that
//! reset loop context.
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
use anyhow::{Result, Error, anyhow, ensure};

use crate::{
    ast::*,
    transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableStatement}
};

#[derive(Debug)]
pub struct BreakStatement;

impl FromRule for BreakStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::break_statement, 
            "Expected `break_statement` rule when building break statement, found: {:?}", pair.as_rule());

        Ok(Some(BreakStatement))
    }
}
impl TranspileableStatement for BreakStatement {}
impl Transpile for BreakStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        if !context.in_loop {
            return Err(vec!(anyhow!(
                "BREAK can only be used inside a WHILE or LOOP block"
            )));
        }

        Ok(TranspilationOutput {
            serialization: "break;".to_string(),
            requested_variables: BTreeSet::new(),
            ..Default::default()
        })
    }
}
