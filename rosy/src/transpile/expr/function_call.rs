use crate::ast::*;
use super::super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error};
use crate::rosy_lib::RosyType;

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
        let function_call_statement = FunctionCallStatement {
            name: self.name.clone(),
            args: self.args.clone()
        };

        let mut output = function_call_statement
            .transpile(context)
            .map_err(|e| e.into_iter().map(|err| {
                err.context(format!("...while transpiling function call to '{}'", self.name))
            }).collect::<Vec<Error>>())?;
        output.serialization = format!("&{}", output.serialization);
        Ok(output)
    }
}