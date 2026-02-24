use std::collections::BTreeSet;
use anyhow::{Result, Error, bail};

use crate::{
    ast::{FromRule, Rule},
    rosy_lib::RosyType,
    transpile::{Transpile, TypeOf, TranspileWithType, TranspilationInputContext, TranspilationOutput}
};

impl FromRule for bool {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::boolean, "Expected boolean rule, got {:?}", pair.as_rule());
        let b = match pair.as_str() {
            "TRUE" => true,
            "FALSE" => false,
            _ => bail!("Unexpected boolean value: {}", pair.as_str()),
        };
        Ok(Some(b))
    }
}
impl TranspileWithType for bool {}
impl TypeOf for bool {
    fn type_of(&self, _context: &TranspilationInputContext) -> Result<RosyType> {
        Ok(RosyType::LO())
    }
}
impl Transpile for bool {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(&self, _context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        Ok(TranspilationOutput {
            serialization: format!("&mut {}", self),
            requested_variables: BTreeSet::new()
        })
    }
}
