//! # OS Statement
//!
//! Triggers a system call, executing a shell command.
//!
//! ## Syntax
//!
//! ```text
//! OS expr;
//! ```
//!
//! ## Rosy Example
//! ```
#![doc = include_str!("test.rosy")]
//! **Output**:
//! ```
#![doc = include_str!("rosy_output.txt")]
//! ## COSY Example
//! ```
#![doc = include_str!("test.fox")]
//! **Output**:
//! ```
#![doc = include_str!("cosy_output.txt")]
//! ```

use std::collections::BTreeSet;
use anyhow::{Result, Context, Error, ensure};

use crate::{
    ast::*, program::expressions::Expr, transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableStatement, add_context_to_all}
};

/// AST node for `OS expr;`.
#[derive(Debug)]
pub struct OsCallStatement {
    pub cmd_expr: Expr,
}

impl FromRule for OsCallStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::os_call,
            "Expected `os_call` rule when building OS statement, found: {:?}", pair.as_rule());

        let mut inner = pair.into_inner();

        let cmd_pair = inner.next()
            .context("Missing command expression in OS!")?;
        let cmd_expr = Expr::from_rule(cmd_pair)
            .context("Failed to build command expression in OS")?
            .ok_or_else(|| anyhow::anyhow!("Expected command expression in OS"))?;

        Ok(Some(OsCallStatement { cmd_expr }))
    }
}
impl TranspileableStatement for OsCallStatement {}
impl Transpile for OsCallStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        let mut requested_variables = BTreeSet::new();

        let cmd_output = self.cmd_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling command expression in OS".to_string()))?;
        requested_variables.extend(cmd_output.requested_variables.iter().cloned());

        let serialization = format!(
            "{{\n    let __os_cmd: String = ({}).to_string();\n    std::process::Command::new(\"sh\").arg(\"-c\").arg(&__os_cmd).status().ok();\n}}",
            cmd_output.as_ref(),
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
            ..Default::default()
        })
    }
}
