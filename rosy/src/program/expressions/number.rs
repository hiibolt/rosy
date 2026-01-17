use std::collections::BTreeSet;
use anyhow::{Result, Error};

use crate::{
    ast::{FromRule, Rule},
    rosy_lib::RosyType,
    transpile::{Transpile, TypeOf, TranspileWithType, TranspilationInputContext, TranspilationOutput}
};

impl FromRule for f64 {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::number, "Expected number rule, got {:?}", pair.as_rule());
        let n = pair.as_str().parse::<f64>()?;
        Ok(Some(n))
    }
}
impl TranspileWithType for f64 {}
impl TypeOf for f64 {
    fn type_of(&self, _context: &TranspilationInputContext) -> Result<RosyType> {
        Ok(RosyType::RE())
    }
}
impl Transpile for f64 {
    fn transpile(&self, _context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        Ok(TranspilationOutput {
            serialization: format!("&mut {}f64", self),
            requested_variables: BTreeSet::new()
        })
    }
}
