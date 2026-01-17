use anyhow::{Result, Context, Error, ensure};

use crate::{
    ast::*,
    transpile::{Transpile, TranspilationInputContext, TranspilationOutput, shared::function_call::function_call_transpile_helper}
};

impl StatementFromRule for FunctionCallStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
        ensure!(pair.as_rule() == Rule::function_call, 
            "Expected `function_call` rule when building function call statement, found: {:?}", pair.as_rule());
        
        let mut inner = pair.into_inner();
        let name = inner.next()
            .context("Missing function name in function call!")?
            .as_str().to_string();
        
        let mut args = Vec::new();
        // Collect all remaining arguments (expressions)
        while let Some(arg_pair) = inner.next() {
            if arg_pair.as_rule() == Rule::semicolon {
                break;
            }
            
            let expr = build_expr(arg_pair)
                .context("Failed to build expression in function call!")?;
            args.push(expr);
        }

        Ok(Some(Statement {
            enum_variant: StatementEnum::FunctionCall,
            inner: Box::new(FunctionCallStatement { name, args })
        }))
    }
}

impl Transpile for FunctionCallStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        function_call_transpile_helper(&self.name, &self.args, context)
    }
}
