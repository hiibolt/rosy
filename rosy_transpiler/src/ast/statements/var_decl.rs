use anyhow::{Result, Context};

use super::super::{Rule, Statement, VariableData, build_type};

pub fn build_var_decl(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();

    let (r#type, dimensions) = build_type(
        inner.next()
            .context("Missing first token `type`!")?
    ).context("...while building variable type in variable declaration!")?;
    let name = inner.next()
        .context("Missing second token `variable_name`!")?
        .as_str().to_string();

    let variable_data = VariableData {
        name,
        r#type,
        dimensions
    };

    Ok(Some(Statement::VarDecl {
        data: variable_data
    }))
}