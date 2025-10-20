use crate::ast::DAInitStatement;
use crate::transpile::{Transpile, TranspilationInputContext, TranspilationOutput};
use anyhow::Error;

impl Transpile for DAInitStatement {
    fn transpile(
        &self, 
        context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        // Transpile the order expression
        let order_output = self.order.transpile(context)
            .map_err(|errs| {
                errs.into_iter()
                    .map(|e| e.context("...while transpiling order expression in DAINI"))
                    .collect::<Vec<_>>()
            })?;
        
        // Transpile the number of variables expression
        let num_vars_output = self.number_of_variables.transpile(context)
            .map_err(|errs| {
                errs.into_iter()
                    .map(|e| e.context("...while transpiling number of variables expression in DAINI"))
                    .collect::<Vec<_>>()
            })?;
        
        let serialization = format!(
            "DA::init(({}).to_owned() as u32, ({}).to_owned() as u32);", 
            order_output.serialization, 
            num_vars_output.serialization
        );
        
        let mut requested_variables = order_output.requested_variables;
        requested_variables.extend(num_vars_output.requested_variables);
        
        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}