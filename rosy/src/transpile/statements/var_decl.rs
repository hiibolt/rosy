use crate::{ast::*, transpile::{ScopedVariableData, VariableData, VariableScope}};
use super::super::{Transpile, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};

impl Transpile for VarDeclStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // Insert the declaration, but check it doesn't already exist
        if matches!(context.variables.insert(self.data.name.clone(), ScopedVariableData { 
            scope: VariableScope::Local,
            data: VariableData { 
                name: self.data.name.clone(),
                r#type: self.data.r#type.clone(),
                total_dimensions: self.data.dimension_exprs.len(),
            }
        }), Some(_)) {
            return Err(vec!(anyhow!("Variable '{}' is already defined in this scope!", self.data.name)));
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