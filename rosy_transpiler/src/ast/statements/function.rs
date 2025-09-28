use anyhow::{Result, Context, ensure};

use rosy_lib::RosyType;
use super::super::{Rule, Statement, VariableData, build_statement};

pub fn build_function(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();
    let (return_type, name, args) = {
        let mut start_function_inner = inner
            .next()
            .context("Missing first token `start_function`!")?
            .into_inner();

        let type_str = start_function_inner.next()
            .context("Missing return type in function declaration!")?
            .as_str().to_string();
        let return_type: RosyType = type_str.as_str().try_into()
            .with_context(|| format!("Unknown type: {type_str}"))?;

        let name = start_function_inner.next()
            .context("Missing function name!")?
            .as_str().to_string();

        let mut args = Vec::new();
        // Collect all remaining argument names and types
        while let Some(arg_pair) = start_function_inner.next() {
            if arg_pair.as_rule() == Rule::semicolon {
                break;
            }

            ensure!(arg_pair.as_rule() == Rule::function_argument_name, 
                "Expected function argument name, found: {:?}", arg_pair.as_rule());
            let name = arg_pair.as_str();

            let next_arg_pair = start_function_inner.next()
                .context(format!("Missing type for function argument: {}", name))?;
            ensure!(next_arg_pair.as_rule() == Rule::r#type, 
                "Expected type for function argument, found: {:?}", next_arg_pair.as_rule());
            let type_str = next_arg_pair.as_str();

            let variable_data = VariableData {
                name: name.to_string(),
                r#type: type_str.try_into()
                    .with_context(|| format!("Unknown type: {type_str}"))?
            };
            args.push(variable_data);
        }

        (return_type, name, args)
    };

    let body = {
        let mut statements = vec!(
            Statement::VarDecl { data: VariableData {
                name: name.clone(),
                r#type: return_type.clone()
            } }
        );

        // Process remaining elements (statements and end_function)
        while let Some(element) = inner.next() {
            // Skip the end_function element
            if element.as_rule() == Rule::end_function {
                break;
            }

            let pair_input = element.as_str();
            if let Some(stmt) = build_statement(element)
                .with_context(|| format!("Failed to build statement from:\n{}", pair_input))? {
                statements.push(stmt);
            }
        }

        statements
    };

    Ok(Some(Statement::Function { name, args, return_type, body }))
}