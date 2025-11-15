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

        // Use DA::variable(usize) to create a DA differential variable
        // Clone the index (which is &mut T) to get an owned value for casting
        let serialization = format!(
            "(&mut DA::variable(({}).clone() as usize)?)",
            index_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables: index_output.requested_variables,
        })
    }
}