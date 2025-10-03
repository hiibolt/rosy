use anyhow::{Result, Context, ensure};

use super::super::{Rule, Statement, VariableData, VarDeclStatement, FunctionStatement, build_statement, build_type};

pub fn build_function(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();
    let (return_type, name, args) = {
        let mut start_function_inner = inner
            .next()
            .context("Missing first token `start_function`!")?
            .into_inner();

        // we choose to ignore the dimensions of the return type for now
        //  since they can be changed dynamically
        //  todo!();
        let (return_type, _) = build_type(
            start_function_inner.next()
                .context("Missing return type for function!")?
        ).context("...while building function return type")?;

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

            let (argument_type, argument_dimensions) = build_type(
                start_function_inner.next()
                    .context(format!("Missing type for function argument: {}", name))?
            ).context("...while building function argument type")?;

            let argument_data = VariableData {
                name: name.to_string(),
                r#type: argument_type,
                dimension_exprs: argument_dimensions,
            };
            args.push(argument_data);
        }

        (return_type, name, args)
    };

    let body = {
        let mut statements = vec!(
            Statement::VarDecl(VarDeclStatement { data: VariableData {
                name: name.clone(),
                r#type: return_type.clone(),
                dimension_exprs: Vec::new(),
            }})
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

    Ok(Some(Statement::Function(FunctionStatement { name, args, return_type, body })))
}