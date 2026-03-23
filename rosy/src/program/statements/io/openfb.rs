//! # OPENFB Statement
//!
//! Opens a **binary** file and associates it with a unit number.
//!
//! ## Syntax
//!
//! ```text
//! OPENFB unit filename status;
//! ```
//!
//! Same arguments as [`super::openf`] but the file is opened in binary mode
//! for use with `WRITEB` / `READB`.
//!
//! ## Example
//!
//! ```text
//! OPENFB 11 'data.bin' 'NEW';
//! WRITEB 11 myVector;
//! CLOSEF 11;
//! ```
//!
//! ```rosy_test_raw
//! --- rosy ---
//! BEGIN;
//!     VARIABLE (VE) V;
//!     V := 1&2&3;
//!     OPENFB 21 'test_openfb_tmp.bin' 'UNKNOWN';
//!     WRITEB 21 V;
//!     CLOSEF 21;
//!     WRITE 6 'openfb ok';
//! END;
//! --- fox ---
//! BEGIN;
//! PROCEDURE RUN;
//!     VARIABLE V 100;
//!     V := 1&2&3;
//!     OPENFB 21 'test_openfb_tmp.bin' 'UNKNOWN';
//!     WRITEB 21 V;
//!     CLOSEF 21;
//!     WRITE 6 'openfb ok';
//! ENDPROCEDURE;
//! RUN;
//! END;
//! ```

use std::collections::BTreeSet;
use anyhow::{Result, Context, Error, ensure};

use crate::{
    ast::*, program::expressions::Expr, transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableStatement, add_context_to_all}
};

/// AST node for `OPENFB unit filename status;`.
/// OPENFB unit filename status ;
#[derive(Debug)]
pub struct OpenfbStatement {
    pub unit_expr: Expr,
    pub filename_expr: Expr,
    pub status_expr: Expr,
}

impl FromRule for OpenfbStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::openfb, 
            "Expected `openfb` rule when building OPENFB statement, found: {:?}", pair.as_rule());
        
        let mut inner = pair.into_inner();

        let unit_pair = inner.next()
            .context("Missing unit expression in OPENFB!")?;
        let unit_expr = Expr::from_rule(unit_pair)
            .context("Failed to build unit expression in OPENFB")?
            .ok_or_else(|| anyhow::anyhow!("Expected unit expression in OPENFB"))?;

        let filename_pair = inner.next()
            .context("Missing filename expression in OPENFB!")?;
        let filename_expr = Expr::from_rule(filename_pair)
            .context("Failed to build filename expression in OPENFB")?
            .ok_or_else(|| anyhow::anyhow!("Expected filename expression in OPENFB"))?;

        let status_pair = inner.next()
            .context("Missing status expression in OPENFB!")?;
        let status_expr = Expr::from_rule(status_pair)
            .context("Failed to build status expression in OPENFB")?
            .ok_or_else(|| anyhow::anyhow!("Expected status expression in OPENFB"))?;

        Ok(Some(OpenfbStatement { unit_expr, filename_expr, status_expr }))
    }
}
impl TranspileableStatement for OpenfbStatement {}
impl Transpile for OpenfbStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        let mut requested_variables = BTreeSet::new();

        let unit_output = self.unit_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling unit expression in OPENFB".to_string()))?;
        requested_variables.extend(unit_output.requested_variables.iter().cloned());

        let filename_output = self.filename_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling filename expression in OPENFB".to_string()))?;
        requested_variables.extend(filename_output.requested_variables.iter().cloned());

        let status_output = self.status_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling status expression in OPENFB".to_string()))?;
        requested_variables.extend(status_output.requested_variables.iter().cloned());

        let serialization = format!(
            "rosy_lib::core::file_io::rosy_openfb({}, {}, {})?;",
            unit_output.as_value(),
            filename_output.as_ref(),
            status_output.as_ref(),
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
            ..Default::default()
        })
    }
}
