use anyhow::{Result, Context};

use super::super::{Rule, Statement, VariableData};

pub fn build_var_decl(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();
    let type_str = inner.next()
        .context("Missing first token `type` when building var decl!")?
        .as_str().to_string();
    let name = inner.next()
        .context("Missing second token `variable_name`!")?
        .as_str().to_string();

    let variable_data = VariableData {
        name,
        r#type: type_str.as_str().try_into()
            .with_context(|| format!("Unknown type: {type_str}"))?
    };

    Ok(Some(Statement::VarDecl {
        data: variable_data
    }))
}