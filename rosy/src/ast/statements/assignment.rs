use anyhow::{Context, Result};

use crate::ast::StatementEnum;

use super::super::{Rule, Statement, AssignStatement, build_variable_identifier, build_expr};

pub fn build_assignment(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();

    let lhs = inner.next()
        .context("Missing first token `variable_name`!")?;
    let identifier = build_variable_identifier(lhs)
        .context("...while building variable identifier for assignment statement")?;

    let expr_pair = inner.next()
        .context("Missing second token `expr`!")?;
    let expr = build_expr(expr_pair)?;

    Ok(Some(Statement { 
        enum_variant: StatementEnum::Assign,
        inner: Box::new(AssignStatement { 
            identifier,
            value: expr
        })
    }))
}
