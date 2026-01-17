use anyhow::{Result, Context, ensure};

use super::super::{Rule, Statement, VariableDeclarationData, ProcedureStatement, build_statement, build_type};

pub fn build_procedure(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
    let mut inner = pair.into_inner();
    let (name, args) = {
        let mut start_procedure_inner = inner
            .next()
            .context("Missing first token `start_procedure`!")?
            .into_inner();

        let name = start_procedure_inner.next()
            .context("Missing procedure name!")?
            .as_str().to_string();
        
        let mut args = Vec::new();
        // Collect all remaining argument names and types
        while let Some(arg_pair) = start_procedure_inner.next() {
            if arg_pair.as_rule() == Rule::semicolon {
                break;
            }
            ensure!(arg_pair.as_rule() == Rule::procedure_argument_name_and_type, 
                "Expected procedure argument name and type, found: {:?}", arg_pair.as_rule());
            
            let mut arg_inner = arg_pair.into_inner();
            let name = arg_inner.next()
                .context("Missing procedure argument name!")?
                .as_str();
            let (r#type, dimension_exprs) = build_type(
                arg_inner.next()
                    .context("Missing procedure argument type!")?
            ).context("...while building procedure argument type")?;
            
            let variable_data = VariableDeclarationData {
                name: name.to_string(),
                r#type,
                dimension_exprs
            };
            args.push(variable_data);
        }

        (name, args)
    };
    
    let body = {
        let mut statements = Vec::new();

        // Process remaining elements (statements and end_procedure)
        while let Some(element) = inner.next() {
            // Skip the end_procedure element
            if element.as_rule() == Rule::end_procedure {
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

    Ok(Some(Statement::Procedure(ProcedureStatement { name, args, body })))
}