use crate::{ast::*, transpile::{TranspileWithType, shared::function_call::function_call_transpile_helper}};
use super::super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error};
use crate::rosy_lib::RosyType;

impl TranspileWithType for FunctionCallExpr {}
impl TypeOf for FunctionCallExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        Ok(context.functions.get(&self.name)
            .ok_or(anyhow::anyhow!("Function '{}' is not defined in this scope, can't call it from expression!", self.name))?
            .return_type
            .clone())
    }
}
impl Transpile for FunctionCallExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        function_call_transpile_helper(&self.name, &self.args, context)
    }
}