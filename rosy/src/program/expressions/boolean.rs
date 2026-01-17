use std::collections::BTreeSet;
use anyhow::{Result, Error};

use crate::{
    rosy_lib::RosyType,
    transpile::{Transpile, TypeOf, TranspileWithType, TranspilationInputContext, TranspilationOutput}
};

impl TranspileWithType for bool {}
impl TypeOf for bool {
    fn type_of(&self, _context: &TranspilationInputContext) -> Result<RosyType> {
        Ok(RosyType::LO())
    }
}
impl Transpile for bool {
    fn transpile(&self, _context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        Ok(TranspilationOutput {
            serialization: format!("&mut {}", self),
            requested_variables: BTreeSet::new()
        })
    }
}
