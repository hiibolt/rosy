use std::collections::BTreeSet;

use anyhow::{Result, Context, Error, anyhow, ensure};

use crate::{
    ast::*, program::expressions::Expr, rosy_lib::{RosyBaseType, RosyType}, transpile::{ScopedVariableData, TranspilationInputContext, TranspilationOutput, Transpile, TypeOf, VariableData, VariableScope}
};

#[derive(Debug)]
pub struct VariableDeclarationData {
    pub name: String,
    pub r#type: Option<RosyType>,
    pub dimension_exprs: Vec<Expr>,
    /// COSY-style memory size expression (e.g. `VARIABLE X 100 ;`)
    /// Parsed for backwards compatibility but never evaluated —
    /// Rosy handles memory allocation automatically.
    pub _memory_size_expr: Option<Expr>,
}
impl VariableDeclarationData {
    /// Helper to unwrap the type or return a descriptive error.
    pub fn require_type(&self) -> Result<RosyType, Error> {
        self.r#type.ok_or_else(|| anyhow!(
            "Type inference is not yet supported - please specify the type for '{}'", self.name
        ))
    }
}
impl Transpile for VariableDeclarationData {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    // note that this transpiles as the default value for the type
    fn transpile (
        &self, context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let resolved_type = self.require_type()
            .map_err(|e| vec![e])?;

        let base_value = match resolved_type.base_type {
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

        // Peek at the next token to see if it's a type or a variable name
        let first = inner.next()
            .context("Missing tokens in variable declaration!")?;

        let (r#type, dimension_exprs, name) = if first.as_rule() == Rule::r#type {
            // Type is present: parse type, then name
            let (r#type, dimension_exprs) = build_type(first)
                .context("...while building variable type in variable declaration!")?;
            let name = inner.next()
                .context("Missing variable name after type in variable declaration!")?
                .as_str().to_string();
            (Some(r#type), dimension_exprs, name)
        } else {
            // No type: first token is the variable name
            let name = first.as_str().to_string();
            (None, Vec::new(), name)
        };

        // Parse optional COSY-style memory size expression (e.g. `VARIABLE X 100 ;`)
        let _memory_size_expr = if let Some(mem_pair) = inner.next() {
            if mem_pair.as_rule() == Rule::memory_size {
                let mem_inner = mem_pair.into_inner().next()
                    .context("Missing expression in memory_size")?;
                let expr = Expr::from_rule(mem_inner)
                    .context("...while building memory size expression in variable declaration!")?;
                if expr.is_some() {
                    eprintln!("Warning: COSY-style memory size in VARIABLE declaration for '{}' is ignored — Rosy handles memory automatically.", name);
                }
                expr
            } else {
                None
            }
        } else {
            None
        };

        let data = VariableDeclarationData {
            name,
            r#type,
            dimension_exprs,
            _memory_size_expr,
        };

        Ok(Some(VarDeclStatement { data }))
    }
}

impl Transpile for VarDeclStatement {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        let resolved_type = self.data.require_type()
            .map_err(|e| vec![e.context(format!("...while transpiling variable declaration for '{}'", self.data.name))])?;

        // Insert the declaration, but check it doesn't already exist
        if matches!(context.variables.insert(self.data.name.clone(), ScopedVariableData { 
            scope: VariableScope::Local,
            data: VariableData { 
                name: self.data.name.clone(),
                r#type: resolved_type.clone()
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
            resolved_type.as_rust_type(),
            data_default_serialization
        );
        Ok(TranspilationOutput {
            serialization,
            requested_variables
        })
    }
}
