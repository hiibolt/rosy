use anyhow::{bail, Context, Result};

use crate::ast::build_variable_identifier;

use super::super::{Rule, Statement, PLoopStatement, build_expr, build_statement};

pub fn build_ploop(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();
    let (iterator, start, end) = {
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

        (iterator, start, end)
    };

    let mut body = Vec::new();
    // Process remaining elements (statements and end)
    let end_ploop_pair = loop {
        let element = inner.next()
            .ok_or(anyhow::anyhow!("Expected `end_ploop` statement at end of `ploop`!"))?;

        // Skip the end element
        if element.as_rule() == Rule::end_ploop {
            break element;
        }

        let pair_input = element.as_str();
        if let Some(stmt) = build_statement(element)
            .with_context(|| format!("Failed to build statement from:\n{}", pair_input))? {
            body.push(stmt);
        }
    };
    let (
        commutivity_rule,
        output
    ) = {
        let mut end_ploop_inner = end_ploop_pair
            .into_inner();

        let first_pair = end_ploop_inner.next()
            .context("Missing first token in `end_ploop` statement!")?;
        let second_pair = end_ploop_inner.next()
            .context("Missing second token in `end_ploop` statement!")?;

        match (first_pair.as_rule(), second_pair.as_rule()) {
            (Rule::unit, Rule::variable_identifier) => {
                let commutivity_rule = first_pair.as_str().parse::<u8>()
                    .context("Failed to parse `commutivity_rule` as u8 in `ploop` statement!")?;
                let output = build_variable_identifier(second_pair)
                    .context("Failed to build `output` variable identifier in `ploop` statement!")?;
                
                (Some(commutivity_rule), output)
            }
            (Rule::variable_identifier, Rule::semicolon) => {
                let output = build_variable_identifier(first_pair)
                    .context("Failed to build `output` variable identifier in `ploop` statement!")?;
                
                (None, output)
            }
            _ => bail!("Expected `variable_identifier` in `end_ploop` statement!"),
        }
    };

    Ok(Some(Statement::PLoop(PLoopStatement {
        iterator,
        start,
        end,
        commutivity_rule,
        body,
        output
    })))  
}