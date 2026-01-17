use std::collections::BTreeSet;
use anyhow::{Result, Error};

use crate::{
    rosy_lib::RosyType,
    transpile::{Transpile, TypeOf, TranspileWithType, TranspilationInputContext, TranspilationOutput}
};

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
