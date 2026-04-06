//! # DACODE Statement (DA Monomial Decode — Not Yet Implemented)
//!
//! Decodes the DA internal monomial numbers to exponent arrays.
//! The first argument is a DA parameter vector (as returned by WRITEM),
//! the second is the array length, and the result is an array where the
//! M-th element contains the exponents of the M-th monomial.
//!
//! **Note:** This procedure exposes COSY's internal monomial numbering scheme.
//! Rosy uses graded lexicographic order which may differ from COSY's internal
//! ordering. Full compatibility requires reconciling the two numbering systems.
//!
//! ## Syntax
//!
//! ```text
//! DACODE params size result;
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

/// AST node for the `DACODE params size result;` monomial decode statement.
#[derive(Debug)]
pub struct DacodeStatement {
    pub params: Expr,
    pub size: Expr,
    pub result: Expr,
}

impl FromRule for DacodeStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(
            pair.as_rule() == Rule::dacode,
            "Expected `dacode` rule when building DACODE statement, found: {:?}",
            pair.as_rule()
        );

        let mut inner = pair.into_inner();

        let params = Expr::from_rule(inner.next().ok_or_else(|| anyhow::anyhow!("Missing params in DACODE"))?)
            .map_err(|e| e)?
            .ok_or_else(|| anyhow::anyhow!("Expected params expression in DACODE"))?;

        let size = Expr::from_rule(inner.next().ok_or_else(|| anyhow::anyhow!("Missing size in DACODE"))?)
            .map_err(|e| e)?
            .ok_or_else(|| anyhow::anyhow!("Expected size expression in DACODE"))?;

        let result = Expr::from_rule(inner.next().ok_or_else(|| anyhow::anyhow!("Missing result in DACODE"))?)
            .map_err(|e| e)?
            .ok_or_else(|| anyhow::anyhow!("Expected result expression in DACODE"))?;

        Ok(Some(DacodeStatement { params, size, result }))
    }
}

impl TranspileableStatement for DacodeStatement {
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

impl Transpile for DacodeStatement {
    fn transpile(
        &self,
        _context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let serialization =
            "anyhow::bail!(\"DACODE is not yet implemented: requires reconciling COSY internal monomial numbering with Rosy graded-lex ordering\");".to_string();

        Ok(TranspilationOutput {
            serialization,
            ..Default::default()
        })
    }
}
