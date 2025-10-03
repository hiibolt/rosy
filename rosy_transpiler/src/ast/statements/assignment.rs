use anyhow::{bail, Context, Result};

use super::super::{Rule, Statement, AssignStatement, build_expr, build_indexed_identifier};

pub fn build_assignment(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();

    let lhs = inner.next()
        .context("Missing first token `variable_name`!")?;
    let (name, indicies) = match lhs.as_rule() {
        Rule::indexed_identifier => {
            build_indexed_identifier(lhs)
                .context("Failed to build indexed identifier in assignment!")?
        },
        Rule::identifier => {
            (lhs.as_str().to_string(), Vec::new())
        },
        other => bail!("Unexpected rule in assignment LHS: {:?}", other),
    };

    let expr_pair = inner.next()
        .context("Missing second token `expr`!")?;
    let expr = build_expr(expr_pair)?;

    Ok(Some(Statement::Assign(AssignStatement { name, value: expr, indicies })))
}
