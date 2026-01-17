use anyhow::{Result, Context, Error, anyhow, ensure};

use crate::{
    ast::*,
    transpile::{Transpile, TranspilationInputContext, TranspilationOutput, ScopedVariableData, VariableData, VariableScope}
};

impl StatementFromRule for VarDeclStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
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

        Ok(Some(Statement {
            enum_variant: StatementEnum::VarDecl,
            inner: Box::new(VarDeclStatement { data })
        }))
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
