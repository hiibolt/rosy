//! # WRITE Statement
//!
//! Writes formatted text to a unit (file or console).
//!
//! ## Syntax
//!
//! ```text
//! WRITE unit expr1 [expr2 ...];
//! ```
//!
//! Unit `6` writes to standard output. Each expression is converted
//! to its string representation and printed.
//!
//! ## Example
//!
//! ```text
//! WRITE 6 'x = ' x ' y = ' y;
//! ```

use std::collections::BTreeSet;
use anyhow::{Result, Context, Error, ensure};

use crate::{
    ast::*, program::{expressions::{Expr, functions::conversion::string_convert::string_convert_transpile_helper}, statements::SourceLocation}, resolve::{ScopeContext, TypeResolver}, transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableStatement, add_context_to_all}
};

/// AST node for the `WRITE unit expr+;` statement.
#[derive(Debug)]
pub struct WriteStatement {
    pub unit: u8,
    pub exprs: Vec<Expr>,
}

impl FromRule for WriteStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::write, 
            "Expected `write` rule when building write statement, found: {:?}", pair.as_rule());
        
        let mut inner = pair.into_inner();

        let unit = inner.next()
            .context("Missing first token `unit`!")?
            .as_str()
            .parse::<u8>()
            .context("Failed to parse `unit` as u8 in `write` statement!")?;

        let exprs = {
            let mut exprs = Vec::new();
            while let Some(expr_pair) = inner.next() {
                if expr_pair.as_rule() == Rule::semicolon {
                    break;
                }

                let expr = Expr::from_rule(expr_pair)
                    .context("Failed to build expression in `write` statement!")?
                    .ok_or_else(|| anyhow::anyhow!("Expected expression in `write` statement"))?;
                exprs.push(expr);
            }
            exprs
        };

        Ok(Some(WriteStatement { unit, exprs }))
    }
}
impl TranspileableStatement for WriteStatement {
    fn discover_dependencies(
        &self,
        resolver: &mut TypeResolver,
        ctx: &mut ScopeContext,
        _source_location: SourceLocation
    ) -> Option<Result<()>> {
        // Discover function call sites within all write expressions
        for expr in &self.exprs {
            if let Err(e) = resolver.discover_expr_function_calls(expr, ctx) {
                return Some(Err(e.context("...while discovering function call dependencies in WRITE statement")));
            }
        }

        Some(Ok(()))
    }
}
impl Transpile for WriteStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        let mut serialized_exprs = Vec::new();
        let mut requested_variables = BTreeSet::new();
        for expr in &self.exprs {
            let TranspilationOutput {
                serialization: serialized_expr,
                requested_variables: expr_requested_variables
            } = string_convert_transpile_helper(expr, context)
                .map_err(|err_vec| {
                    add_context_to_all(err_vec, format!(
                        "...while transpiling expression '{:?}' for WRITE statement", expr
                    ))
                })?;

            serialized_exprs.push(serialized_expr);
            requested_variables.extend(expr_requested_variables);
        }

        // Emulate the checking of the unit
        match self.unit {
            6 => {
                // Write to stdout
                let serialization = format!(
                    "println!(\"{}\", {});",
                    serialized_exprs.iter().map(|_| "{}").collect::<Vec<&str>>().join(""),
                    serialized_exprs.join(", ")
                );
                Ok(TranspilationOutput {
                    serialization,
                    requested_variables
                })
            },
            unit => {
                // Write to file unit
                let mut stmts = Vec::new();
                for expr_ser in &serialized_exprs {
                    stmts.push(format!(
                        "rosy_lib::core::file_io::rosy_write_to_unit({}, &format!(\"{{}}\", {}))?;",
                        unit,
                        expr_ser
                    ));
                }
                Ok(TranspilationOutput {
                    serialization: stmts.join("\n"),
                    requested_variables
                })
            },
        }
    }
}
