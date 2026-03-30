//! # DAEPS Statement (DA Epsilon / Cutoff Threshold)
//!
//! Sets the garbage collection tolerance (cutoff threshold) for coefficients
//! of DA and CD vectors.
//!
//! ## Syntax
//!
//! ```text
//! DAEPS c;
//! ```
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

use crate::{
    ast::*,
    program::{expressions::Expr, statements::SourceLocation},
    resolve::{ScopeContext, TypeResolver},
    transpile::{
        TranspilationInputContext, TranspilationOutput, Transpile, TranspileableStatement,
        TypeslotDeclarationResult,
    },
};

/// AST node for the `DAEPS c;` DA epsilon statement.
#[derive(Debug)]
pub struct DaepsStatement {
    pub epsilon: Expr,
}

impl FromRule for DaepsStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(
            pair.as_rule() == Rule::daeps,
            "Expected `daeps` rule when building DAEPS statement, found: {:?}",
            pair.as_rule()
        );

        let mut inner = pair.into_inner();

        let epsilon_pair = inner
            .next()
            .context("Missing epsilon parameter in DAEPS statement!")?;
        let epsilon_expr = Expr::from_rule(epsilon_pair)
            .context("Failed to build epsilon expression in DAEPS statement!")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for epsilon in DAEPS statement"))?;

        Ok(Some(DaepsStatement {
            epsilon: epsilon_expr,
        }))
    }
}
impl TranspileableStatement for DaepsStatement {
    fn register_typeslot_declaration(
        &self,
        _resolver: &mut TypeResolver,
        _ctx: &mut ScopeContext,
        _source_location: SourceLocation,
    ) -> TypeslotDeclarationResult {
        TypeslotDeclarationResult::NotAVarFuncOrProcedureDecl
    }
}
impl Transpile for DaepsStatement {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let epsilon_output = self.epsilon.transpile(context).map_err(|errs| {
            errs.into_iter()
                .map(|e| e.context("...while transpiling epsilon expression in DAEPS"))
                .collect::<Vec<_>>()
        })?;

        let serialization = format!(
            "taylor::set_epsilon({} as f64)?;",
            epsilon_output.as_value()
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables: epsilon_output.requested_variables,
            ..Default::default()
        })
    }
}
