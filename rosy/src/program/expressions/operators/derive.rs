use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileWithType, TypeOf};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;

/// DA%n = partial derivative w.r.t. variable n (positive n)
/// DA%(-n) = anti-derivative (integral) w.r.t. variable n (negative n)
#[derive(Debug, PartialEq)]
pub struct DeriveExpr {
    pub object: Box<Expr>,
    pub index: Box<Expr>,
}

impl TranspileWithType for DeriveExpr {}
impl Transpile for DeriveExpr {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let object_output = self.object.transpile(context)?;
        let index_output = self.index.transpile(context)?;

        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(object_output.requested_variables);
        requested_variables.extend(index_output.requested_variables);

        // Generate code that checks the sign of the index at runtime:
        // positive => derivative, negative => antiderivative
        let serialization = format!(
            "&mut RosyDerive::rosy_derive(&*{}, ({}).clone() as i64)?",
            object_output.serialization,
            index_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TypeOf for DeriveExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        let object_type = self.object.type_of(context)
            .context("Failed to determine type of object in % (derive) expression")?;
        
        match object_type {
            t if t == RosyType::DA() => Ok(RosyType::DA()),
            t if t == RosyType::CD() => Ok(RosyType::CD()),
            _ => anyhow::bail!(
                "Derivation operator % not supported for type: {:?}. Only DA and CD are supported.",
                object_type
            ),
        }
    }
}
