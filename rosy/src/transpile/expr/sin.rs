use crate::ast::SinExpr;
use crate::transpile::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;

impl Transpile for SinExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        // Transpile the inner expression
        let inner_output = self.expr.transpile(context)?;
        
        // Combine requested variables
        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(inner_output.requested_variables);

        // Generate the transpiled code
        let serialization = format!(
            "&mut {}.rosy_sin()?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}

impl TypeOf for SinExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::sin;
        
        // Get the type of the inner expression
        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in SIN")?;
        
        // Use the SIN registry to get the return type
        sin::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "SIN not supported for type: {:?}",
                    inner_type
                )
            })
    }
}
