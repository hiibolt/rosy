//! # SUBSTR Statement
//!
//! Extracts a substring from a string.
//!
//! ## Syntax
//!
//! ```text
//! SUBSTR source first last destination;
//! ```
//!
//! - `source` — string expression to extract from
//! - `first` — 1-indexed start position (RE)
//! - `last` — 1-indexed end position (RE), inclusive
//! - `destination` — variable (ST) to write the result into
//!
//! ## Example
//!
//! ```text
//! VARIABLE (ST) s;
//! VARIABLE (ST) sub;
//! s := 'Hello, World!';
//! SUBSTR s 1 5 sub;
//! WRITE 6 sub;   { prints: Hello }
//! ```

use std::collections::BTreeSet;
use anyhow::{Result, Context, Error, ensure};

use crate::{
    ast::*,
    program::expressions::{Expr, core::variable_identifier::VariableIdentifier},
    transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableStatement, add_context_to_all, VariableScope},
};

/// AST node for `SUBSTR source first last destination;`.
#[derive(Debug)]
pub struct SubstrStatement {
    pub source_expr: Expr,
    pub first_expr: Expr,
    pub last_expr: Expr,
    pub dest: VariableIdentifier,
}

impl FromRule for SubstrStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::substr,
            "Expected `substr` rule when building SUBSTR statement, found: {:?}", pair.as_rule());

        let mut inner = pair.into_inner();

        let source_pair = inner.next()
            .context("Missing source expression in SUBSTR!")?;
        let source_expr = Expr::from_rule(source_pair)
            .context("Failed to build source expression in SUBSTR")?
            .ok_or_else(|| anyhow::anyhow!("Expected source expression in SUBSTR"))?;

        let first_pair = inner.next()
            .context("Missing first-position expression in SUBSTR!")?;
        let first_expr = Expr::from_rule(first_pair)
            .context("Failed to build first-position expression in SUBSTR")?
            .ok_or_else(|| anyhow::anyhow!("Expected first-position expression in SUBSTR"))?;

        let last_pair = inner.next()
            .context("Missing last-position expression in SUBSTR!")?;
        let last_expr = Expr::from_rule(last_pair)
            .context("Failed to build last-position expression in SUBSTR")?
            .ok_or_else(|| anyhow::anyhow!("Expected last-position expression in SUBSTR"))?;

        let dest_pair = inner.next()
            .context("Missing destination variable in SUBSTR!")?;
        let dest = VariableIdentifier::from_rule(dest_pair)
            .context("Failed to build destination variable in SUBSTR")?
            .ok_or_else(|| anyhow::anyhow!("Expected destination variable in SUBSTR"))?;

        Ok(Some(SubstrStatement { source_expr, first_expr, last_expr, dest }))
    }
}

impl TranspileableStatement for SubstrStatement {}

impl Transpile for SubstrStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        let mut requested_variables = BTreeSet::new();

        let source_output = self.source_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling source expression in SUBSTR".to_string()))?;
        requested_variables.extend(source_output.requested_variables);

        let first_output = self.first_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling first-position expression in SUBSTR".to_string()))?;
        requested_variables.extend(first_output.requested_variables);

        let last_output = self.last_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling last-position expression in SUBSTR".to_string()))?;
        requested_variables.extend(last_output.requested_variables);

        let dest_output = self.dest.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling destination variable in SUBSTR".to_string()))?;
        requested_variables.extend(dest_output.requested_variables);

        // Determine if we need to dereference the destination (Arg or Higher scope)
        let dereference = match context.variables.get(&self.dest.name)
            .ok_or_else(|| vec![anyhow::anyhow!("Variable '{}' is not defined in this scope!", self.dest.name)])?
            .scope
        {
            VariableScope::Local => "",
            VariableScope::Arg => "*",
            VariableScope::Higher => {
                requested_variables.insert(self.dest.name.clone());
                "*"
            }
        };

        let serialization = format!(
            r#"{{
    let __substr_src = ({}).to_owned();
    let __substr_first = ({}).to_owned() as usize;
    let __substr_last = ({}).to_owned() as usize;
    {}{} = __substr_src.get((__substr_first - 1)..__substr_last)
        .unwrap_or("")
        .to_string();
}}"#,
            source_output.serialization,
            first_output.serialization,
            last_output.serialization,
            dereference,
            dest_output.serialization,
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
