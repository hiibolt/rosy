use crate::ast::*;
use super::super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error};
use crate::rosy_lib::RosyType;

impl TypeOf for ComplexExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        let expr_type = self.expr.type_of(context)
            .map_err(|e| e.context("...while determining type of expression for complex conversion"))?;
        let result_type = crate::rosy_lib::intrinsics::cm::get_return_type(&expr_type)
            .ok_or(anyhow::anyhow!(
                "Cannot convert type '{}' to 'CM'!",
                expr_type
            ))?;
        Ok(result_type)
    }
}
impl Transpile for ComplexExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // First, ensure the type is convertible to CM
        //
        // Sneaky way to check that the type is compatible :3
        let _ = self.type_of(context)
            .map_err(|e| vec!(e.context("...while verifying types of complex conversion expression")))?;

        // Then, transpile the expression
        let TranspilationOutput {
            serialization: expr_serialization,
            requested_variables
        } = self.expr.transpile(context)
            .map_err(|e| e.into_iter().map(|err| {
                err.context("...while transpiling expression for CM conversion")
            }).collect::<Vec<Error>>())?;

        // Finally, serialize the conversion
        let serialization = format!("&mut RosyCM::rosy_cm(&*{}).context(\"...while trying to convert to (CM)\")?", expr_serialization);
        Ok(TranspilationOutput {
            serialization,
            requested_variables
        })
    }
}