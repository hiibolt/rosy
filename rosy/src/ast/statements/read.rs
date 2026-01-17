use anyhow::{Result, Context};

use crate::ast::StatementEnum;

use super::super::{Rule, Statement, ReadStatement, build_variable_identifier};

pub fn build_read(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();

    let unit = inner.next()
        .context("Missing first token `unit`!")?
        .as_str()
        .parse::<u8>()
        .context("Failed to parse `unit` as u8 in `read` statement!")?;

    let identifier = build_variable_identifier(
        inner.next()
        .context("Missing second token `variable_identifier`!")?
    ).context("...while building variable identifier for read statement")?;

    Ok(Some(Statement {
        enum_variant: StatementEnum::Read,
        inner: Box::new(ReadStatement { unit, identifier })
    }))
}
