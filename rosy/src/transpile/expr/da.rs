use crate::ast::DAExpr;
use crate::transpile::{Transpile, TranspilationInputContext, TranspilationOutput, TypeOf};
use anyhow::Error;
use crate::rosy_lib::RosyType;

impl TypeOf for DAExpr {
    fn type_of(&self, _context: &TranspilationInputContext) -> anyhow::Result<RosyType> {
        Ok(RosyType::DA())
    }
}

impl Transpile for DAExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        // Transpile the index expression
        let index_output = self.index.transpile(context)
            .map_err(|errs| {
                errs.into_iter()
                    .map(|e| e.context("...while transpiling DA index expression"))
                    .collect::<Vec<_>>()
            })?;

        // Use DA::from(f64) to create a DA from a variable index
        // The dace library implements From<f64> for DA
        let serialization = format!("DA::from(({}).to_owned())", index_output.serialization);

        Ok(TranspilationOutput {
            serialization,
            requested_variables: index_output.requested_variables,
        })
    }
}