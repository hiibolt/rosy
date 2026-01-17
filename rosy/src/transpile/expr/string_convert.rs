use crate::{ast::*, transpile::{TranspileWithType, shared::string_convert::string_convert_transpile_helper}};
use super::super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error};
use crate::rosy_lib::RosyType;

impl TranspileWithType for StringConvertExpr {}
impl TypeOf for StringConvertExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        let expr_type = self.expr.type_of(context)?;
        crate::rosy_lib::intrinsics::st::get_return_type(&expr_type)
            .ok_or(anyhow::anyhow!("Cannot convert type '{expr_type}' to 'ST'!"))
    }
}
impl Transpile for StringConvertExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        string_convert_transpile_helper(&self.expr, context)
    }
}