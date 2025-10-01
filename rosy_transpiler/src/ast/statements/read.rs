use anyhow::{Result, Context};

use super::super::{Rule, Statement};

pub fn build_read(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();

    let unit = inner.next()
        .context("Missing first token `unit`!")?
        .as_str()
        .parse::<u8>()
        .context("Failed to parse `unit` as u8 in `read` statement!")?;

    let name = inner.next()
        .context("Missing second token `variable_name`!")?
        .as_str()
        .to_string();

    Ok(Some(Statement::Read { unit, name }))
}
