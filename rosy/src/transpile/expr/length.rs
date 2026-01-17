use crate::ast::LengthExpr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileWithType, TypeOf};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;

impl TranspileWithType for LengthExpr {}
impl Transpile for LengthExpr {
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
            "&mut {}.rosy_length()",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TypeOf for LengthExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::length;
        
        // Get the type of the inner expression
        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in LENGTH")?;
        
        // Use the LENGTH registry to get the return type
        length::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "LENGTH not supported for type: {:?}",
                    inner_type
                )
            })
    }
}