//! # CDFLO Statement (Complex DA Flow — Not Yet Implemented)
//!
//! Same as DAFLO but with complex DA (CD) arguments.
//! Computes the flow of x' = f(x) for time step 1 via iterated Lie series,
//! where f and the result are complex DA vectors.
//!
//! **Note:** Requires a complex Lie series ODE solver. Planned future work.
//!
//! ## Syntax
//!
//! ```text
//! CDFLO rhs ic result dim;
//! ```

use anyhow::{Error, Result, ensure};

use crate::{
    ast::*,
    program::{expressions::Expr, statements::SourceLocation},
    resolve::{ScopeContext, TypeResolver},
    transpile::{
        InferenceEdgeResult, TranspilationInputContext, TranspilationOutput, Transpile,
        TranspileableStatement, TypeHydrationResult, TypeslotDeclarationResult,
    },
};

/// AST node for the `CDFLO rhs ic result dim;` complex ODE flow statement.
#[derive(Debug)]
pub struct CdfloStatement {
    pub rhs: Expr,
    pub ic: Expr,
    pub result: Expr,
    pub dim: Expr,
}

impl FromRule for CdfloStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(
            pair.as_rule() == Rule::cdflo,
            "Expected `cdflo` rule when building CDFLO statement, found: {:?}",
            pair.as_rule()
        );

        let mut inner = pair.into_inner();

        let rhs = Expr::from_rule(inner.next().ok_or_else(|| anyhow::anyhow!("Missing rhs in CDFLO"))?)
            .map_err(|e| e)?
            .ok_or_else(|| anyhow::anyhow!("Expected rhs expression in CDFLO"))?;

        let ic = Expr::from_rule(inner.next().ok_or_else(|| anyhow::anyhow!("Missing ic in CDFLO"))?)
            .map_err(|e| e)?
            .ok_or_else(|| anyhow::anyhow!("Expected ic expression in CDFLO"))?;

        let result = Expr::from_rule(inner.next().ok_or_else(|| anyhow::anyhow!("Missing result in CDFLO"))?)
            .map_err(|e| e)?
            .ok_or_else(|| anyhow::anyhow!("Expected result expression in CDFLO"))?;

        let dim = Expr::from_rule(inner.next().ok_or_else(|| anyhow::anyhow!("Missing dim in CDFLO"))?)
            .map_err(|e| e)?
            .ok_or_else(|| anyhow::anyhow!("Expected dim expression in CDFLO"))?;

        Ok(Some(CdfloStatement { rhs, ic, result, dim }))
    }
}

impl TranspileableStatement for CdfloStatement {
    fn register_typeslot_declaration(
        &self,
        _resolver: &mut TypeResolver,
        _ctx: &mut ScopeContext,
        _source_location: SourceLocation,
    ) -> TypeslotDeclarationResult {
        TypeslotDeclarationResult::NotAVarFuncOrProcedureDecl
    }
    fn wire_inference_edges(
        &self,
        _resolver: &mut TypeResolver,
        _ctx: &mut ScopeContext,
        _source_location: SourceLocation,
    ) -> InferenceEdgeResult {
        InferenceEdgeResult::NoEdges
    }
    fn hydrate_resolved_types(
        &mut self,
        _resolver: &TypeResolver,
        _current_scope: &[String],
    ) -> TypeHydrationResult {
        TypeHydrationResult::NothingToHydrate
    }
}

impl Transpile for CdfloStatement {
    fn transpile(
        &self,
        _context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let serialization =
            "anyhow::bail!(\"CDFLO is not yet implemented: requires a complex Lie series ODE flow solver\");".to_string();

        Ok(TranspilationOutput {
            serialization,
            ..Default::default()
        })
    }
}
