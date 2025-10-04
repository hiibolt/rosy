use std::collections::BTreeSet;

use crate::ast::*;
use super::{Transpile, TranspilationInputContext, TranspilationOutput, VariableScope};
use anyhow::{Result, Error, anyhow};


impl Transpile for ReadStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        let mut requested_variables = BTreeSet::new();
        
        // Check that the requested variable name
        let var_data = context.variables.get(&self.var_name)
            .ok_or(vec!(anyhow!(
                "Variable '{}' is not defined in this scope!", self.var_name
            )))?;
        match var_data.scope {
            VariableScope::Higher => {
                requested_variables.insert(self.var_name.clone());
            },
            VariableScope::Arg => {},
            VariableScope::Local => {}
        }

        // Check that the variable is of a supported type
        let variable_type = &var_data.data.r#type;
        let return_type = rosy_lib::intrinsics::from_st::get_return_type(variable_type)
            .ok_or(vec!(anyhow!(
                "READ doesn't support reading into variables of type '{}', found variable '{}' of type '{}'", 
                variable_type, self.var_name, variable_type
            )))?;

        // Emulate the checking of the unit
        match self.unit {
            5 => {},
            _ => return Err(vec!(anyhow!(
                "Only READ from unit 5 (standard input) is supported, found unit {}!", self.unit
            ))),
        }

        // Serialize the entire function
        let serialization = format!(
            "{} = rosy_lib::intrinsics::from_st::from_stdin::<{}>().context(\"Failed to READ into {}\")?;",
            self.var_name, return_type.as_rust_type(), self.var_name
        );
        Ok(TranspilationOutput {
            serialization,
            requested_variables
        })
    }
}