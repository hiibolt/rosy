//! # WRITEB Statement (Binary Write)
//!
//! Writes values in binary format to a file unit.
//!
//! ## Syntax
//!
//! ```text
//! WRITEB unit expr1 [expr2 ...];
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
use std::collections::BTreeSet;

use crate::{
    ast::*,
    program::{expressions::Expr, statements::SourceLocation},
    resolve::{ScopeContext, TypeResolver},
    transpile::{
        TranspilationInputContext, TranspilationOutput, Transpile, TranspileableStatement,
        TypeslotDeclarationResult, add_context_to_all,
    },
};

/// AST node for `WRITEB unit expr+;`.
/// WRITEB unit expr+ ;
#[derive(Debug)]
pub struct WritebStatement {
    pub unit: u8,
    pub exprs: Vec<Expr>,
}

impl FromRule for WritebStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(
            pair.as_rule() == Rule::writeb,
            "Expected `writeb` rule when building WRITEB statement, found: {:?}",
            pair.as_rule()
        );

        let mut inner = pair.into_inner();

        let unit = inner
            .next()
            .context("Missing first token `unit`!")?
            .as_str()
            .parse::<u8>()
            .context("Failed to parse `unit` as u8 in `writeb` statement!")?;

        let exprs = {
            let mut exprs = Vec::new();
            while let Some(expr_pair) = inner.next() {
                if expr_pair.as_rule() == Rule::semicolon {
                    break;
                }

                let expr = Expr::from_rule(expr_pair)
                    .context("Failed to build expression in `writeb` statement!")?
                    .ok_or_else(|| anyhow::anyhow!("Expected expression in `writeb` statement"))?;
                exprs.push(expr);
            }
            exprs
        };

        Ok(Some(WritebStatement { unit, exprs }))
    }
}
impl TranspileableStatement for WritebStatement {
    fn register_typeslot_declaration(
        &self,
        _resolver: &mut TypeResolver,
        _ctx: &mut ScopeContext,
        _source_location: SourceLocation,
    ) -> TypeslotDeclarationResult {
        TypeslotDeclarationResult::NotAVarFuncOrProcedureDecl
    }
}
impl Transpile for WritebStatement {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let mut serialized_stmts = Vec::new();
        let mut requested_variables = BTreeSet::new();

        for expr in &self.exprs {
            let output = expr.transpile(context).map_err(|err_vec| {
                add_context_to_all(
                    err_vec,
                    format!(
                        "...while transpiling expression '{:?}' for WRITEB statement",
                        expr
                    ),
                )
            })?;

            requested_variables.extend(output.requested_variables.iter().cloned());

            // Use RosyToBinary trait to convert to bytes, then write
            serialized_stmts.push(format!(
                "rosy_lib::core::file_io::rosy_writeb_to_unit({}, &rosy_lib::core::file_io::RosyToBinary::to_binary({}))?;",
                self.unit,
                output.as_ref()
            ));
        }

        Ok(TranspilationOutput {
            serialization: serialized_stmts.join("\n"),
            requested_variables,
            ..Default::default()
        })
    }
}
