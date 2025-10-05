use crate::ast::*;
use super::super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput, VariableScope, };
use anyhow::{Result, Context, Error};
use rosy_lib::RosyType;

impl TypeOf for VarExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        self.identifier.type_of(context)
            .context(format!(
                "...while determining type of variable identifier '{}'", self.identifier.name
            ))
    }
}
impl Transpile for VarExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        let TranspilationOutput {
            serialization: serialized_identifier,
            requested_variables
        } = self.identifier.transpile(context)
            .map_err(|e| e.into_iter().map(|err| {
                err.context(format!(
                    "...while transpiling variable identifier '{}'", self.identifier.name
                ))
            }).collect::<Vec<Error>>())?;
        
        let reference = match context.variables.get(&self.identifier.name)
            .ok_or(vec!(anyhow::anyhow!("Variable '{}' is not defined in this scope!", self.identifier.name)))? 
            .scope
        {
            VariableScope::Local => "&mut ",
            VariableScope::Arg => "",
            VariableScope::Higher => ""
        };
        Ok(TranspilationOutput {
            serialization: format!("{}{}", reference, serialized_identifier),
            requested_variables
        })
    }
}