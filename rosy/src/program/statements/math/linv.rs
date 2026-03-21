//! # LINV Statement
//!
//! Inverts a quadratic matrix.
//!
//! ## Syntax
//!
//! ```text
//! LINV matrix inverse n alloc_dim error_flag;
//! ```
//!
//! - `matrix`     — input matrix (2D RE array, `Vec<Vec<f64>>`)
//! - `inverse`    — variable to receive the output inverse matrix
//! - `n`          — number of actual entries (RE, used as usize)
//! - `alloc_dim`  — allocation dimension (RE, used as usize)
//! - `error_flag` — variable to receive error code (0: no error, 132: singular)

use std::collections::BTreeSet;
use anyhow::{Result, Context, Error, ensure};

use crate::{
    ast::*, program::expressions::Expr,
    transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableStatement, add_context_to_all}
};

/// AST node for `LINV matrix inverse n alloc_dim error_flag;`.
#[derive(Debug)]
pub struct LinvStatement {
    pub matrix_expr: Expr,
    pub inverse_expr: Expr,
    pub n_expr: Expr,
    pub alloc_dim_expr: Expr,
    pub error_flag_expr: Expr,
}

impl FromRule for LinvStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::linv,
            "Expected `linv` rule when building LINV statement, found: {:?}", pair.as_rule());

        let mut inner = pair.into_inner();

        let matrix_pair = inner.next()
            .context("Missing matrix parameter in LINV!")?;
        let matrix_expr = Expr::from_rule(matrix_pair)
            .context("Failed to build matrix expression in LINV")?
            .ok_or_else(|| anyhow::anyhow!("Expected matrix expression in LINV"))?;

        let inverse_pair = inner.next()
            .context("Missing inverse parameter in LINV!")?;
        let inverse_expr = Expr::from_rule(inverse_pair)
            .context("Failed to build inverse expression in LINV")?
            .ok_or_else(|| anyhow::anyhow!("Expected inverse expression in LINV"))?;

        let n_pair = inner.next()
            .context("Missing n parameter in LINV!")?;
        let n_expr = Expr::from_rule(n_pair)
            .context("Failed to build n expression in LINV")?
            .ok_or_else(|| anyhow::anyhow!("Expected n expression in LINV"))?;

        let alloc_dim_pair = inner.next()
            .context("Missing alloc_dim parameter in LINV!")?;
        let alloc_dim_expr = Expr::from_rule(alloc_dim_pair)
            .context("Failed to build alloc_dim expression in LINV")?
            .ok_or_else(|| anyhow::anyhow!("Expected alloc_dim expression in LINV"))?;

        let error_flag_pair = inner.next()
            .context("Missing error_flag parameter in LINV!")?;
        let error_flag_expr = Expr::from_rule(error_flag_pair)
            .context("Failed to build error_flag expression in LINV")?
            .ok_or_else(|| anyhow::anyhow!("Expected error_flag expression in LINV"))?;

        Ok(Some(LinvStatement {
            matrix_expr,
            inverse_expr,
            n_expr,
            alloc_dim_expr,
            error_flag_expr,
        }))
    }
}

impl TranspileableStatement for LinvStatement {}

impl Transpile for LinvStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        let mut requested_variables = BTreeSet::new();

        let matrix_output = self.matrix_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling matrix in LINV".to_string()))?;
        requested_variables.extend(matrix_output.requested_variables);

        let inverse_output = self.inverse_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling inverse in LINV".to_string()))?;
        requested_variables.extend(inverse_output.requested_variables.clone());

        let n_output = self.n_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling n in LINV".to_string()))?;
        requested_variables.extend(n_output.requested_variables);

        let alloc_dim_output = self.alloc_dim_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling alloc_dim in LINV".to_string()))?;
        requested_variables.extend(alloc_dim_output.requested_variables);

        let error_flag_output = self.error_flag_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling error_flag in LINV".to_string()))?;
        requested_variables.extend(error_flag_output.requested_variables.clone());

        // The inverse and error_flag are output (write) targets.
        // VarExpr::transpile for a local variable produces `&name`; for higher/arg scopes it
        // produces just `name`.
        // - Local: strip leading `&` → bare name, assign directly: `name = value`
        // - Higher/Arg: no leading `&` → bare name already, assign via deref: `*name = value`
        fn make_lvalue_assign(serialization: &str, value: &str) -> String {
            if let Some(bare) = serialization.strip_prefix('&') {
                format!("{bare} = {value}")
            } else {
                format!("*{serialization} = {value}")
            }
        }

        let inverse_assign = make_lvalue_assign(&inverse_output.serialization, "rosy_linv_inv");
        let error_assign = make_lvalue_assign(&error_flag_output.serialization, "rosy_linv_err");

        let serialization = format!(
            "{{ let (rosy_linv_inv, rosy_linv_err) = rosy_lib::core::linv::rosy_linv(&*{matrix}, ({n}).to_owned() as usize, ({alloc_dim}).to_owned() as usize)?; {inverse_assign}; {error_assign}; }}",
            matrix = matrix_output.serialization,
            n = n_output.serialization,
            alloc_dim = alloc_dim_output.serialization,
            inverse_assign = inverse_assign,
            error_assign = error_assign,
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
            ..Default::default()
        })
    }
}
