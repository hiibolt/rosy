//! # DAFLO Statement (DA Flow — Not Yet Implemented)
//!
//! Computes the DA representation of the flow of x' = f(x) for time step 1
//! to nearly machine accuracy via iterated Lie series.
//!
//! **Note:** Implementing DAFLO requires a Lie series solver for autonomous ODEs.
//! This is algorithmically complex and is planned as future work.
//!
//! ## Syntax
//!
//! ```text
//! DAFLO rhs ic result dim;
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

/// AST node for the `DAFLO rhs ic result dim;` ODE flow statement.
#[derive(Debug)]
pub struct DafloStatement {
    pub rhs: Expr,
    pub ic: Expr,
    pub result: Expr,
    pub dim: Expr,
}

impl FromRule for DafloStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(
            pair.as_rule() == Rule::daflo,
            "Expected `daflo` rule when building DAFLO statement, found: {:?}",
            pair.as_rule()
        );

        let mut inner = pair.into_inner();

        let rhs = Expr::from_rule(inner.next().ok_or_else(|| anyhow::anyhow!("Missing rhs in DAFLO"))?)
            .map_err(|e| e)?
            .ok_or_else(|| anyhow::anyhow!("Expected rhs expression in DAFLO"))?;

        let ic = Expr::from_rule(inner.next().ok_or_else(|| anyhow::anyhow!("Missing ic in DAFLO"))?)
            .map_err(|e| e)?
            .ok_or_else(|| anyhow::anyhow!("Expected ic expression in DAFLO"))?;

        let result = Expr::from_rule(inner.next().ok_or_else(|| anyhow::anyhow!("Missing result in DAFLO"))?)
            .map_err(|e| e)?
            .ok_or_else(|| anyhow::anyhow!("Expected result expression in DAFLO"))?;

        let dim = Expr::from_rule(inner.next().ok_or_else(|| anyhow::anyhow!("Missing dim in DAFLO"))?)
            .map_err(|e| e)?
            .ok_or_else(|| anyhow::anyhow!("Expected dim expression in DAFLO"))?;

        Ok(Some(DafloStatement { rhs, ic, result, dim }))
    }
}

impl TranspileableStatement for DafloStatement {
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

impl Transpile for DafloStatement {
    fn transpile(
        &self,
        _context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let serialization =
            "anyhow::bail!(\"DAFLO is not yet implemented: requires a Lie series ODE flow solver\");".to_string();

        Ok(TranspilationOutput {
            serialization,
            ..Default::default()
        })
    }
}
