use anyhow::{Result, Context};

use crate::ast::StatementEnum;

use super::super::{Rule, Statement, WriteStatement, build_expr};

pub fn build_write(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();

    let unit = inner.next()
        .context("Missing first token `unit`!")?
        .as_str()
        .parse::<u8>()
        .context("Failed to parse `unit` as u8 in `write` statement!")?;

    let exprs = {
        let mut exprs = Vec::new();
        while let Some(expr_pair) = inner.next() {
            if expr_pair.as_rule() == Rule::semicolon {
                break;
            }

            let expr = build_expr(expr_pair)
                .context("Failed to build expression in `write` statement!")?;
            exprs.push(expr);
        }
        exprs
    };

    Ok(Some(Statement {
        enum_variant: StatementEnum::Write,
        inner: Box::new(WriteStatement { unit, exprs })
    }))
}
