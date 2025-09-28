use anyhow::{Result, Context};

use super::super::{Rule, Statement, build_expr};

pub fn build_assignment(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();
    let name = inner.next()
        .context("Missing first token `variable_name`!")?
        .as_str().to_string();
    let expr_pair = inner.next()
        .context("Missing second token `expr`!")?;
    let expr = build_expr(expr_pair)?;
    Ok(Some(Statement::Assign { name, value: expr }))
}
