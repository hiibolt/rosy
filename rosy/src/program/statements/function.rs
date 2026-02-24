use std::collections::BTreeSet;
use anyhow::{Result, Context, Error, anyhow, ensure};

use crate::{
    ast::*, rosy_lib::RosyType, program::statements::*, transpile::{ScopedVariableData, TranspilationInputContext, TranspilationInputFunctionContext, TranspilationOutput, Transpile, VariableData, VariableScope, indent}
};

#[derive(Debug)]
pub struct FunctionStatement {
    pub name: String,
    pub args: Vec<VariableDeclarationData>,
    pub return_type: Option<RosyType>,
    pub body: Vec<Statement>
}

impl FromRule for FunctionStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::function, 
            "Expected `function` rule when building function statement, found: {:?}", pair.as_rule());
        
        let mut inner = pair.into_inner();
        let (return_type, name, args) = {
            let mut start_function_inner = inner
                .next()
                .context("Missing first token `start_function`!")?
                .into_inner();

            // Return type is now optional - peek to see if the next token is a type or a name
            let first = start_function_inner.next()
                .context("Missing tokens in function declaration!")?;

            let (return_type, name) = if first.as_rule() == Rule::r#type {
                // we choose to ignore the dimensions of the return type for now
                //  since they can be changed dynamically
                let (return_type, _) = build_type(first)
                    .context("...while building function return type")?;
                let name = start_function_inner.next()
                    .context("Missing function name!")?
                    .as_str().to_string();
                (Some(return_type), name)
            } else {
                // No return type specified, first token is the function name
                let name = first.as_str().to_string();
                (None, name)
            };

            let mut args = Vec::new();
            // Collect all remaining argument names and types
            // With optional types, we peek at each token:
            //   - function_argument_name followed by type → typed argument
            //   - function_argument_name followed by another name or semicolon → untyped argument
            while let Some(arg_pair) = start_function_inner.next() {
                if arg_pair.as_rule() == Rule::semicolon {
                    break;
                }

                ensure!(arg_pair.as_rule() == Rule::function_argument_name, 
                    "Expected function argument name, found: {:?}", arg_pair.as_rule());
                let arg_name = arg_pair.as_str();

                // Peek at the next token to see if it's a type
                let next = start_function_inner.peek();
                let (argument_type, argument_dimensions) = match next {
                    Some(ref p) if p.as_rule() == Rule::r#type => {
                        let type_pair = start_function_inner.next().unwrap();
                        let (t, d) = build_type(type_pair)
                            .context("...while building function argument type")?;
                        (Some(t), d)
                    },
                    _ => (None, Vec::new())
                };

                let argument_data = VariableDeclarationData {
                    name: arg_name.to_string(),
                    r#type: argument_type,
                    dimension_exprs: argument_dimensions,
                };
                args.push(argument_data);
            }

            (return_type, name, args)
        };

        let body = {
            let mut statements = vec!(
                Statement {
                    enum_variant: StatementEnum::VarDecl,
                    inner: Box::new(VarDeclStatement {
                        data: VariableDeclarationData {
                            name: name.clone(),
                            r#type: return_type.clone(),
                            dimension_exprs: Vec::new(),
                        }
                    })
                }
            );

            // Process remaining elements (statements and end_function)
            while let Some(element) = inner.next() {
                // Skip the end_function element
                if element.as_rule() == Rule::end_function {
                    break;
                }

                let pair_input = element.as_str();
                if let Some(stmt) = Statement::from_rule(element)
                    .with_context(|| format!("Failed to build statement from:\n{}", pair_input))? {
                    statements.push(stmt);
                }
            }

            statements
        };

        Ok(Some(FunctionStatement { name, args, return_type, body }))
    }
}

impl Transpile for FunctionStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        // Resolve the return type (required for transpilation)
        let resolved_return_type = self.return_type
            .ok_or_else(|| anyhow!("Type inference is not yet supported - please specify the return type for function '{}'", self.name))
            .map_err(|e| vec![e])?;

        // Resolve all argument types (required for transpilation)
        let resolved_arg_data: Vec<VariableData> = {
            let mut data = Vec::new();
            let mut errors = Vec::new();
            for arg in &self.args {
                match arg.require_type() {
                    Ok(t) => data.push(VariableData {
                        name: arg.name.clone(),
                        r#type: t,
                    }),
                    Err(e) => errors.push(e.context(format!(
                        "...while resolving argument types for function '{}'", self.name
                    ))),
                }
            }
            if !errors.is_empty() {
                return Err(errors);
            }
            data
        };

        // Insert the function signature, but check it doesn't already exist
        if context.functions.contains_key(&self.name) ||
            matches!(context.functions.insert(
                    self.name.clone(),
                    TranspilationInputFunctionContext {
                        return_type: resolved_return_type.clone(),
                        args: resolved_arg_data.clone(),
                        requested_variables: BTreeSet::new()
                    }
                ), Some(_))
        {
            return Err(vec!(anyhow!("Function '{}' is already defined in this scope!", self.name)));
        }


        // Define and raise the level of any existing variables
        let mut inner_context: TranspilationInputContext = context.clone();
        inner_context.in_loop = false;
        let mut requested_variables = BTreeSet::new();
        let mut serialized_statements = Vec::new();
        let mut errors = Vec::new();
        for (_, ScopedVariableData { scope, .. }) in inner_context.variables.iter_mut() {
            *scope = match *scope {
                VariableScope::Local => VariableScope::Higher,
                VariableScope::Arg => VariableScope::Higher,
                VariableScope::Higher => VariableScope::Higher
            }
        }
        for arg_data in &resolved_arg_data {
                if matches!(inner_context.variables.insert(arg_data.name.clone(), ScopedVariableData {
                scope: VariableScope::Arg,
                data: arg_data.clone()
            }), Some(_)) {
                errors.push(anyhow!("Argument '{}' is already defined!", arg_data.name));
            }
        }

        // Transpile each inner statement
        for stmt in &self.body {
            match stmt.transpile(&mut inner_context) {
                Ok(output) => {
                    serialized_statements.push(output.serialization);
                    requested_variables.extend(output.requested_variables);
                },
                Err(stmt_errors) => {
                    for e in stmt_errors {
                        errors.push(e.context(format!(
                            "...while transpiling statement in function '{}'", self.name
                        )));
                    }
                }
            }
        }

        // Update the function context with the requested variables
        if let Some(func_context) = context.functions.get_mut(&self.name) {
            func_context.requested_variables = requested_variables.clone();
        } else {
            errors.push(anyhow!(
                "Function '{}' was not found in context after being inserted!", self.name
            ).context("...while updating function context"));
        }

        // Serialize arguments
        let serialized_args: Vec<String> = {
            let mut serialized_args = Vec::new();
            for var_name in requested_variables.iter() {
                let Some(var_data) = inner_context.variables
                    .get(var_name) else 
                {
                    errors.push(anyhow!(
                        "Variable '{}' was requested but not found in context!", var_name
                    ).context(format!(
                        "...while transpiling function '{}'", self.name
                    )));
                    continue;
                };

                serialized_args.push(format!(
                    "{}: &mut {}",
                    var_name,
                    var_data.data.r#type.as_rust_type()
                ));
            }
            for arg_data in &resolved_arg_data {
                serialized_args.push(format!(
                    "{}: &{}",
                    arg_data.name,
                    arg_data.r#type.as_rust_type()
                ));
            }
            serialized_args
        };

        // Serialize return type
        let serialized_return_type = resolved_return_type.as_rust_type();

        // Serialize the entire function
        let serialization = format!(
            "fn {} ( {} ) -> Result<{}> {{\n{}\n\tOk({})\n}}",
            self.name, serialized_args.join(", "), 
            serialized_return_type,
            indent(serialized_statements.join("\n")),
            self.name
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
