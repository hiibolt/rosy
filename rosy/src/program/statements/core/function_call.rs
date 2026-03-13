//! # Function Call Statement
//!
//! Calls a user-defined function as a statement (return value is discarded).
//! This is the statement-level counterpart to function call expressions.
//!
//! ## Syntax
//!
//! ```text
//! FNAME arg1 [arg2 ...];
//! ```

use anyhow::{Result, Context, Error, ensure};

use crate::{
    ast::*, program::{expressions::{Expr, core::var_expr::function_call_transpile_helper}, statements::SourceLocation}, resolve::{ScopeContext, TypeResolver}, transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableStatement}
};

/// AST node for a function call used as a statement.
#[derive(Debug)]
pub struct FunctionCallStatement {
    pub name: String,
    pub args: Vec<Expr>,
}

impl FromRule for FunctionCallStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
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
            
            let expr = Expr::from_rule(arg_pair)
                .context("Failed to build expression in function call!")?
                .ok_or_else(|| anyhow::anyhow!("Expected expression in function call"))?;
            args.push(expr);
        }

        Ok(Some(FunctionCallStatement { name, args }))
    }
}
impl TranspileableStatement for FunctionCallStatement {
    fn discover_dependencies(
        &self,
        resolver: &mut TypeResolver,
        ctx: &mut ScopeContext,
        _source_location: SourceLocation
    ) -> Option<Result<()>> {
        Some(resolver.discover_call_site_deps(&self.name, &self.args, true, ctx))
    }
}
impl Transpile for FunctionCallStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        function_call_transpile_helper(&self.name, &self.args, context)
    }
}
