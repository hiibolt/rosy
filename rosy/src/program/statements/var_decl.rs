use std::collections::BTreeSet;

use anyhow::{Result, Context, Error, anyhow, ensure};

use crate::{
    ast::*, program::expressions::Expr, rosy_lib::{RosyBaseType, RosyType}, transpile::{ScopedVariableData, TranspilationInputContext, TranspilationOutput, Transpile, TypeOf, VariableData, VariableScope}
};

#[derive(Debug)]
pub struct VariableDeclarationData {
    pub name: String,
    pub r#type: RosyType,
    pub dimension_exprs: Vec<Expr>
}
impl Transpile for VariableDeclarationData {
    // note that this transpiles as the default value for the type
    fn transpile (
        &self, context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let base_value = match self.r#type.base_type {
            RosyBaseType::RE => "0.0",
            RosyBaseType::ST => "\"\".to_string()",
            RosyBaseType::LO => "false",
            RosyBaseType::CM => "Complex64::new(0.0, 0.0)",
            RosyBaseType::VE => "vec![]",
            RosyBaseType::DA => "DA::zero()",
            RosyBaseType::CD => "CD::zero()"
        }.to_string();

        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();
        let serialization = if self.dimension_exprs.is_empty() {
            base_value
        } else {
            let mut result = base_value;
            for dim in self.dimension_exprs.iter().rev() {
                // ensure the type compiles down to a RE
                if let Err(e) = dim.type_of(context).and_then(|t| {
                    let expected_type = RosyType::RE();
                    if t == expected_type {
                        Ok(())
                    } else {
                        Err(anyhow::anyhow!("Array dimension expression must be of type {expected_type}, found {t}"))
                    }
                }) {
                    errors.push(e.context("...while checking array dimension expression type"));
                    continue;
                }

                // transpile each dimension expression
                match dim.transpile(context) {
                    Ok(output) => {
                        result = format!("vec![{}; ({}).to_owned() as usize]", result, output.serialization);
                        requested_variables.extend(output.requested_variables);
                    },
                    Err(dim_errors) => {
                        for e in dim_errors {
                            errors.push(e.context("...while transpiling array dimension expression!"));
                        }
                    }
                }
            }
            result
        };

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

#[derive(Debug)]
pub struct VarDeclStatement {
    pub data: VariableDeclarationData
}

impl FromRule for VarDeclStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::var_decl, 
            "Expected `var_decl` rule when building variable declaration, found: {:?}", pair.as_rule());
        
        let mut inner = pair.into_inner();

        let (r#type, dimension_exprs) = build_type(
            inner.next()
                .context("Missing first token `type`!")?
        ).context("...while building variable type in variable declaration!")?;
        
        let name = inner.next()
            .context("Missing second token `variable_name`!")?
            .as_str().to_string();

        let data = VariableDeclarationData {
            name,
            r#type,
            dimension_exprs
        };

        Ok(Some(VarDeclStatement { data }))
    }
}

impl Transpile for VarDeclStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        // Insert the declaration, but check it doesn't already exist
        if matches!(context.variables.insert(self.data.name.clone(), ScopedVariableData { 
            scope: VariableScope::Local,
            data: VariableData { 
                name: self.data.name.clone(),
                r#type: self.data.r#type.clone()
            }
        }), Some(_)) {
            return Err(vec!(anyhow!("Variable '{}' is already defined in this scope!", self.data.name)));
        }

        let TranspilationOutput { 
            serialization: data_default_serialization,
            requested_variables 
        } = self.data.transpile(context)?;

        let serialization = format!(
            "let mut {}: {} = {};",
            &self.data.name,
            self.data.r#type.as_rust_type(),
            data_default_serialization
        );
        Ok(TranspilationOutput {
            serialization,
            requested_variables
        })
    }
}
