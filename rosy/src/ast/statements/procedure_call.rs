use anyhow::{Result, Context};

use super::super::{Rule, Statement, ProcedureCallStatement, build_expr};

pub fn build_procedure_call(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();
    let name = inner.next()
        .context("Missing procedure name in procedure call!")?
        .as_str().to_string();
    
    let mut args = Vec::new();
    // Collect all remaining arguments (expressions)
    while let Some(arg_pair) = inner.next() {
        if arg_pair.as_rule() == Rule::semicolon {
            break;
        }
        
        let expr = build_expr(arg_pair)
            .context("Failed to build expression in procedure call!")?;
        args.push(expr);
    }
    
    Ok(Some(Statement::ProcedureCall(ProcedureCallStatement { name, args })))
}