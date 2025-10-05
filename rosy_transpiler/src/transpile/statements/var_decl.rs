use crate::ast::*;
use super::super::{Transpile, TranspilationInputContext, TranspilationOutput, ScopedVariableData, VariableScope};
use anyhow::{Result, Error, anyhow};

impl Transpile for VarDeclStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // Insert the declaration, but check it doesn't already exist
        if matches!(context.variables.insert(self.data.name.clone(), ScopedVariableData { 
            scope: VariableScope::Local,
            data: self.data.clone()
        }), Some(_)) {
            return Err(vec!(anyhow!(
                "Variable '{}' is already defined in this scope!", self.data.name
            )));
        }

        let TranspilationOutput { 
            serialization: data_default_serialization,
            requested_variables 
        } = self.data.transpile(context)?;

        let serialization = format!(
            "let mut {}: {} = {};",
            &self.data.name,
            self.data.r#type.as_rust_type(),
            data_default_serialization
        );
        Ok(TranspilationOutput {
            serialization,
            requested_variables
        })
    }
}