use crate::ast::*;
use super::super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};
use crate::rosy_lib::RosyType;

impl TypeOf for LogicalConvertExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        let expr_type = self.expr.type_of(context)?;
        crate::rosy_lib::intrinsics::lo::get_return_type(&expr_type)
            .ok_or(anyhow::anyhow!("Cannot convert type '{expr_type}' to 'LO'!"))
    }
}
impl Transpile for LogicalConvertExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // First, ensure the type is convertible to LO
        let expr_type = self.expr.type_of(context)
            .map_err(|e| vec!(e))?;
        let _ = crate::rosy_lib::intrinsics::lo::get_return_type(&expr_type)
            .ok_or(vec!(anyhow!(
                "Cannot convert type '{}' to 'LO'!", expr_type
            )))?;

        // Then, transpile the expression
        let TranspilationOutput {
            serialization: expr_serialization,
            requested_variables
        } = self.expr.transpile(context)
            .map_err(|e| e.into_iter().map(|err| {
                err.context("...while transpiling expression for LO conversion")
            }).collect::<Vec<Error>>())?;

        // Finally, serialize the conversion
        let serialization = format!("&mut RosyLO::rosy_to_logical(&*{})", expr_serialization);
        Ok(TranspilationOutput {
            serialization,
            requested_variables
        })
    }
}