//! # ST — String Literal
//!
//! String literals enclosed in single quotes. Escaped single quotes
//! are written as `''` inside the string.
//!
//! ## Syntax
//!
//! ```text
//! 'hello world'
//! 'it''s escaped'
//! ```
//!
//! All string literals produce the `ST` type.

use std::collections::{BTreeSet, HashSet};
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use anyhow::{Result, Error};

use crate::{
    ast::{FromRule, Rule},
    rosy_lib::RosyType,
    transpile::{Transpile, TranspileableExpr, TranspilationInputContext, TranspilationOutput}
};

impl FromRule for String {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::string, "Expected string rule, got {:?}", pair.as_rule());
        let s = pair.as_str();

        // Remove the surrounding quotes
        let s = &s[1..s.len()-1];

        // Handle escaped single quotes: '' -> '
        let s = s.replace("''", "'");

        Ok(Some(s.to_string()))
    }
}
impl TranspileableExpr for String {
    fn type_of(&self, _context: &TranspilationInputContext) -> Result<RosyType> {
        Ok(RosyType::ST())
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::ST()))
    }
}
impl Transpile for String {
    fn transpile(&self, _context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        Ok(TranspilationOutput {
            serialization: format!("&mut String::from(\"{}\")", self),
            requested_variables: BTreeSet::new(),
            ..Default::default()
        })
    }
}
