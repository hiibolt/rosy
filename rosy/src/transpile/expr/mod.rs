mod var_expr;
mod concat;
mod complex;
mod extract;
mod add;
mod sub;
mod mult;
mod div;
mod string_convert;
mod logical_convert;
mod function_call;
mod da;
mod length;
mod sin;

use crate::{ast::*, rosy_lib::RosyType, transpile::{TranspileWithType, TypeOf, add_context_to_all}};
use super::{Transpile, TranspilationInputContext, TranspilationOutput};
use std::collections::BTreeSet;
use anyhow::{Result, Error};

impl TranspileWithType for f64 {}
impl TypeOf for f64 {
    fn type_of ( &self, _context: &TranspilationInputContext ) -> Result<RosyType> {
        Ok(RosyType::RE())
    }
}
impl Transpile for f64 {
    fn transpile (
        &self, _context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        Ok(TranspilationOutput {
            serialization: format!("&mut ({}f64)", self),
            requested_variables: BTreeSet::new(),
        })
    }
}

impl TranspileWithType for String {}
impl TypeOf for String {
    fn type_of ( &self, _context: &TranspilationInputContext ) -> Result<RosyType> {
        Ok(RosyType::ST())
    }
}
impl Transpile for String {
    fn transpile (
        &self, _context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        Ok(TranspilationOutput {
            serialization: format!("&mut \"{}\".to_string()", self.replace('"', "\\\"")),
            requested_variables: BTreeSet::new(),
        })
    }
}

impl TranspileWithType for bool {}
impl TypeOf for bool {
    fn type_of ( &self, _context: &TranspilationInputContext ) -> Result<RosyType> {
        Ok(RosyType::LO())
    }
}
impl Transpile for bool {
    fn transpile (
        &self, _context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        Ok(TranspilationOutput {
            serialization: format!("&mut {}", if *self { "true" } else { "false" }),
            requested_variables: BTreeSet::new(),
        })
    }
}
impl Transpile for Expr {
    fn transpile (
        &self, context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        self.inner.transpile(context)
            .map_err(|err_vec| {
                add_context_to_all(err_vec, format!("...while transpiling expression: {:?}", self))
            })
    }
}