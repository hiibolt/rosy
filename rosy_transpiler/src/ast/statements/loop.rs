use anyhow::{Result, Context};

use super::super::{Rule, Statement, LoopStatement, build_expr, build_statement};

pub fn build_loop(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();
    let (iterator, start, end, step) = {
        let mut start_loop_inner = inner
            .next()
            .context("Missing first token `start_loop`!")?
            .into_inner();

        let iterator = start_loop_inner.next()
            .context("Missing first token `variable_name`!")?
            .as_str().to_string();
        let start_pair = start_loop_inner.next()
            .context("Missing second token `start_expr`!")?;
        let start = build_expr(start_pair)
            .context("Failed to build `start` expression in `loop` statement!")?;
        let end_pair = start_loop_inner.next()
            .context("Missing third token `end_expr`!")?;
        let end = build_expr(end_pair)
            .context("Failed to build `end` expression in `loop` statement!")?;
        
        // Optional step expression
        let step = if let Some(step_pair) = start_loop_inner.next() {
            if step_pair.as_rule() == Rule::expr {
                Some(build_expr(step_pair)
                    .context("Failed to build `step` expression in `loop` statement!")?)
            } else {
                None
            }
        } else {
            None
        };

        (iterator, start, end, step)
    };

    let mut body = Vec::new();
    // Process remaining elements (statements and end)
    while let Some(element) = inner.next() {
        // Skip the end element
        if element.as_rule() == Rule::end_loop {
            break;
        }

        let pair_input = element.as_str();
        if let Some(stmt) = build_statement(element)
            .with_context(|| format!("Failed to build statement from:\n{}", pair_input))? {
            body.push(stmt);
        }
    }

    Ok(Some(Statement::Loop(LoopStatement { iterator, start, end, step, body })))  
}