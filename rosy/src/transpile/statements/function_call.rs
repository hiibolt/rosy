use crate::{ast::*, transpile::shared::function_call::function_call_transpile_helper};
use super::super::{Transpile, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error};

impl Transpile for FunctionCallStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        function_call_transpile_helper(&self.name, &self.args, context)
    }
}