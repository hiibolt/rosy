use std::collections::BTreeSet;

use crate::ast::Rule;
use crate::{ast::FromRule, transpile::*};
use crate::rosy_lib::RosyType;
use crate::program::expressions::Expr;
use anyhow::{Result, Context, Error, ensure};

#[derive(Debug, PartialEq)]
pub struct VariableIdentifier {
    pub name: String,
    pub indicies: Vec<Expr>
}

impl FromRule for VariableIdentifier {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<VariableIdentifier>> {
        ensure!(pair.as_rule() == Rule::variable_identifier, 
            "Expected `variable_identifier` rule when building variable identifier, found: {:?}", pair.as_rule());
            
        let mut inner = pair.into_inner();
        let name = inner.next()
            .context("Missing variable name in indexed identifier!")?
            .as_str().to_string();
        
        let indicies = if let Some(next) = inner.next() {
            let mut indices = Vec::new();
            let mut inner_indices = next.into_inner();
            while let Some(index_pair) = inner_indices.next() {
                let expr = Expr::from_rule(index_pair)
                    .context("Failed to build expression in indexed identifier!")?
                    .ok_or_else(|| anyhow::anyhow!("Expected expression in indexed identifier"))?;
                indices.push(expr);
            }
            indices
        } else {
            Vec::new()
        };

        Ok(Some(VariableIdentifier {
            name,
            indicies
        }))
    }
}
impl TypeOf for VariableIdentifier {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        let var_data = context.variables.get(&self.name)
            .ok_or(anyhow::anyhow!("Variable '{}' is not defined in this scope!", self.name))?;

        let mut var_type = var_data.data.r#type.clone();
        var_type.dimensions = var_type.dimensions
            .checked_sub(self.indicies.len())
            .ok_or(anyhow::anyhow!(
                "Variable '{}' does not have enough dimensions to index into it (tried to index {} times, but it only has {} dimensions)!",
                self.name, self.indicies.len(), var_type.dimensions
            ))?;

        Ok(var_type)
    }
}
impl Transpile for VariableIdentifier {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // Check that the variable exists and that the 
        //  dimensions are correct
        //
        // Cheeky trick to reuse code :3
        let _ = self.type_of(context)
            .map_err(|err| {
                vec!(err.context(format!("...while checking the type of variable {}", self.name)))
            })?;

        // Serialize the indicies
        let mut serialized_indicies = String::new();
        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();
        for (i, index_expr) in self.indicies.iter().enumerate() {
            let i = i + 1;
            let name = &self.name;

            // Check that the type is RE
            let index_expr_type = index_expr.type_of(context)
                .map_err(|err| {
                    vec!(err.context(format!("...while checking the type for index expression {i} of {name}")))
                })?;
            let expected_type = RosyType::RE();
            if index_expr_type != expected_type {
                return Err(vec!(anyhow::anyhow!("Indexing expression {i} when indexing {name} was {index_expr_type}, when it should be {expected_type}!")));
            }

            // Transpile it
            match index_expr.transpile(context) {
                Ok(output) => {
                    serialized_indicies.push_str(&format!("[(({}).to_owned() - 1.0f64) as usize]", output.serialization));
                    requested_variables.extend(output.requested_variables);
                },
                Err(vec_err) => {
                    for err in vec_err {
                        errors.push(err.context(format!(
                            "...while transpiling index expression to {}", self.name
                        )));
                    }
                }
            }
        }

        // Finally, serialize the entire variable
        if VariableScope::Higher == context.variables.get(&self.name)
            .ok_or(vec!(anyhow::anyhow!("Variable '{}' is not defined in this scope!", self.name)))? 
            .scope
        {
            requested_variables.insert(self.name.clone());
        }
        let serialization = format!(
            "{}{}",
            self.name,
            serialized_indicies
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