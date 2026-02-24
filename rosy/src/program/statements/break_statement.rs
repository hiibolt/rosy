//! BREAK statement implementation.
//!
//! Syntax: `BREAK;`
//!
//! Exits the innermost enclosing WHILE or LOOP block immediately.
//! Only valid inside WHILE or LOOP contexts - not valid inside PLOOP,
//! and PROCEDURE/FUNCTION definitions create scope boundaries that
//! reset loop context.

use std::collections::BTreeSet;
use anyhow::{Result, Error, anyhow, ensure};

use crate::{
    ast::*,
    transpile::{TranspilationInputContext, TranspilationOutput, Transpile}
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

impl Transpile for BreakStatement {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        if !context.in_loop {
            return Err(vec!(anyhow!(
                "BREAK can only be used inside a WHILE or LOOP block"
            )));
        }

        Ok(TranspilationOutput {
            serialization: "break;".to_string(),
            requested_variables: BTreeSet::new(),
        })
    }
}
