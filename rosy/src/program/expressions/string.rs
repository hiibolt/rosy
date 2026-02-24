use std::collections::BTreeSet;
use anyhow::{Result, Error};

use crate::{
    ast::{FromRule, Rule},
    rosy_lib::RosyType,
    transpile::{Transpile, TypeOf, TranspileWithType, TranspilationInputContext, TranspilationOutput}
};

impl FromRule for String {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::string, "Expected string rule, got {:?}", pair.as_rule());
        let s = pair.as_str();

        // Remove the surrounding quotes
        let s = &s[1..s.len()-1];

        Ok(Some(s.to_string()))
    }
}
impl TranspileWithType for String {}
impl TypeOf for String {
    fn type_of(&self, _context: &TranspilationInputContext) -> Result<RosyType> {
        Ok(RosyType::ST())
    }
}
impl Transpile for String {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(&self, _context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        Ok(TranspilationOutput {
            serialization: format!("&mut String::from(\"{}\")", self),
            requested_variables: BTreeSet::new()
        })
    }
}
