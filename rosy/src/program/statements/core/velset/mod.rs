//! # VELSET Statement
//!
//! Sets a component of a vector of reals (VE) to a given value.
//!
//! ## Syntax
//!
//! ```text
//! VELSET vector component value;
//! ```
//!
//! - `vector`    — VE variable to modify in place (must be a plain variable name)
//! - `component` — RE expression for the component number (1-indexed)
//! - `value`     — RE expression for the value to assign
//!
//! ## Rosy Example
//! ```
#![doc = include_str!("test.rosy")]
//! ```
//! **Output**:
//! ```
#![doc = include_str!("rosy_output.txt")]
//! ```
//! ## COSY Example
//! ```
#![doc = include_str!("test.fox")]
//! ```
//! **Output**:
//! ```
#![doc = include_str!("cosy_output.txt")]
//! ```

use anyhow::{Context, Error, Result, ensure};
use std::collections::BTreeSet;

use crate::{
    ast::*,
    program::expressions::{Expr, core::variable_identifier::VariableIdentifier},
    transpile::{
        TranspilationInputContext, TranspilationOutput, Transpile, TranspileableStatement,
        add_context_to_all,
    },
};

/// AST node for `VELSET vector component value;`.
#[derive(Debug)]
pub struct VelsetStatement {
    pub vector_ident: VariableIdentifier,
    pub component_expr: Expr,
    pub value_expr: Expr,
}

impl FromRule for VelsetStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(
            pair.as_rule() == Rule::velset,
            "Expected `velset` rule when building VELSET statement, found: {:?}",
            pair.as_rule()
        );

        let mut inner = pair.into_inner();

        let vector_pair = inner.next().context("Missing vector variable in VELSET!")?;
        let vector_ident = VariableIdentifier::from_rule(vector_pair)
            .context("Failed to build vector variable identifier in VELSET")?
            .ok_or_else(|| anyhow::anyhow!("Expected vector variable identifier in VELSET"))?;

        let component_pair = inner
            .next()
            .context("Missing component expression in VELSET!")?;
        let component_expr = Expr::from_rule(component_pair)
            .context("Failed to build component expression in VELSET")?
            .ok_or_else(|| anyhow::anyhow!("Expected component expression in VELSET"))?;

        let value_pair = inner
            .next()
            .context("Missing value expression in VELSET!")?;
        let value_expr = Expr::from_rule(value_pair)
            .context("Failed to build value expression in VELSET")?
            .ok_or_else(|| anyhow::anyhow!("Expected value expression in VELSET"))?;

        Ok(Some(VelsetStatement {
            vector_ident,
            component_expr,
            value_expr,
        }))
    }
}

impl TranspileableStatement for VelsetStatement {}

impl Transpile for VelsetStatement {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let mut requested_variables = BTreeSet::new();

        // Transpile the vector identifier to get the variable name (as a plain name, no & prefix).
        let vector_output = self.vector_ident.transpile(context).map_err(|e| {
            add_context_to_all(
                e,
                "...while transpiling vector identifier in VELSET".to_string(),
            )
        })?;
        requested_variables.extend(vector_output.requested_variables.clone());
        // vector_output.serialization is just the variable name (e.g. "v"), suitable as an l-value.
        let vec_name = vector_output.serialization;

        let component_output = self.component_expr.transpile(context).map_err(|e| {
            add_context_to_all(
                e,
                "...while transpiling component expression in VELSET".to_string(),
            )
        })?;
        requested_variables.extend(component_output.requested_variables.iter().cloned());

        let value_output = self.value_expr.transpile(context).map_err(|e| {
            add_context_to_all(
                e,
                "...while transpiling value expression in VELSET".to_string(),
            )
        })?;
        requested_variables.extend(value_output.requested_variables.iter().cloned());

        // COSY uses 1-indexed components; convert to 0-indexed for Rust.
        // vec_name is a plain variable name so it can be used as a mutable l-value directly.
        let serialization = format!(
            "{{ let __velset_idx = ({} as usize).checked_sub(1).expect(\"VELSET component index must be >= 1\"); {}[__velset_idx] = {}; }}",
            component_output.as_value(),
            vec_name,
            value_output.as_value(),
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
            ..Default::default()
        })
    }
}
