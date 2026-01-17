use std::collections::BTreeSet;
use anyhow::{Result, Error};

use crate::{
    rosy_lib::RosyType,
    transpile::{Transpile, TypeOf, TranspileWithType, TranspilationInputContext, TranspilationOutput}
};

impl TranspileWithType for String {}

impl TypeOf for String {
    fn type_of(&self, _context: &TranspilationInputContext) -> Result<RosyType> {
        Ok(RosyType::ST())
    }
}

impl Transpile for String {
    fn transpile(&self, _context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        Ok(TranspilationOutput {
            serialization: format!("&mut String::from(\"{}\")", self),
            requested_variables: BTreeSet::new()
        })
    }
}
