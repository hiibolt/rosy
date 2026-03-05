use std::collections::BTreeSet;
use anyhow::{Result, Context, Error, anyhow, ensure};

use crate::{
    ast::*, program::expressions::Expr, transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TypeOf, VariableScope}
};

#[derive(Debug)]
pub struct ProcedureCallStatement {
    pub name: String,
    pub args: Vec<Expr>,
}

impl FromRule for ProcedureCallStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::procedure_call, 
            "Expected `procedure_call` rule when building procedure call statement, found: {:?}", pair.as_rule());
        
        let mut inner = pair.into_inner();
        let name = inner.next()
            .context("Missing procedure name in procedure call!")?
            .as_str().to_string();
        
        let mut args = Vec::new();
        // Collect all remaining arguments (expressions)
        while let Some(arg_pair) = inner.next() {
            if arg_pair.as_rule() == Rule::semicolon {
                break;
            }
            
            let expr = Expr::from_rule(arg_pair)
                .context("Failed to build expression in procedure call!")?
                .ok_or_else(|| anyhow::anyhow!("Expected expression in procedure call"))?;
            args.push(expr);
        }
        
        Ok(Some(ProcedureCallStatement { name, args }))
    }
}

impl Transpile for ProcedureCallStatement {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        // Start by checking that the procedure exists
        let proc_context = match context.procedures.get(&self.name) {
            Some(ctx) => ctx,
            None => return Err(vec!(anyhow!("procedure '{}' is not defined in this scope, can't transpile procedure call!", self.name)))
        }.clone();

        // Check that the number of arguments is correct
        if proc_context.args.len() != self.args.len() {
            return Err(vec!(anyhow!(
                "procedure '{}' expects {} arguments, but {} were provided!",
                self.name, proc_context.args.len(), self.args.len()
            )));
        }
        let mut errors = Vec::new();
        let mut requested_variables = BTreeSet::new();
        let mut serialized_args = Vec::new();
        // Serialize the requested variables from the procedure context
        for var in &proc_context.requested_variables {
            let var_data = context.variables.get(var)
                .ok_or(vec!(anyhow!(
                    "Could not find variable '{}' requested by procedure '{}'",
                    var, self.name
                )))?;
            
            let serialized_arg = match var_data.scope {
                VariableScope::Higher => format!("{}", var),
                VariableScope::Arg => format!("{}", var),
                VariableScope::Local => format!("&mut {}", var)
            };
            serialized_args.push(serialized_arg);
        }

        // Add the manual arguments
        for (i, arg_expr) in self.args.iter().enumerate() {
            match arg_expr.transpile(context) {
                Ok(arg_output) => {
                    // Check the type is correct
                    let provided_type = arg_expr.type_of(context)
                        .map_err(|e| vec!(e))?;
                    let expected_type = proc_context
                        .args
                        .get(i)
                        .ok_or(vec!(anyhow!(
                            "procedure '{}' expects {} arguments, but {} were provided!",
                            self.name, proc_context.args.len(), self.args.len()
                        )))?
                        .r#type
                        .clone();
                    if provided_type != expected_type {
                        errors.push(anyhow!(
                            "procedure '{}' expects argument {} ('{}') to be of type '{}', but type '{}' was provided!",
                            self.name, i+1, proc_context.args[i].name, expected_type, provided_type
                        ));
                    } else {
                        // If the type is correct, add the serialization
                        serialized_args.push(format!("&mut {}", arg_output.serialization));
                        requested_variables.extend(arg_output.requested_variables);
                    }
                },
                Err(arg_errors) => {
                    for e in arg_errors {
                        errors.push(e.context(format!(
                            "...while transpiling argument {} for procedure '{}'", i+1, self.name
                        )));
                    }
                }
            }
        }

        // Serialize the entire procedure
        let serialization = format!(
            "{}({}).context(\"...while calling procedure '{}'\")?;",
            self.name, serialized_args.join(", "), self.name
        );
        if errors.is_empty() {
            Ok(TranspilationOutput {
                serialization,
                requested_variables
            })
        } else {
            Err(errors)
        }
    }
}
