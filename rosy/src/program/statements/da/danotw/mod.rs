//! # DANOTW Statement (DA Weighted Order — Not Yet Implemented)
//!
//! Sets the weighted order factor of each independent variable for DA and CD.
//! Must be called before DAINI. This affects how the "total order" constraint
//! is computed during monomial enumeration.
//!
//! **Note:** Full support requires rearchitecting the monomial enumeration in
//! `init_taylor` to accept per-variable weight factors. This is planned work.
//!
//! ## Syntax
//!
//! ```text
//! DANOTW weights size;
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

/// AST node for the `DANOTW weights size;` weighted order statement.
#[derive(Debug)]
pub struct DanotwStatement {
    pub weights: Expr,
    pub size: Expr,
}

impl FromRule for DanotwStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(
            pair.as_rule() == Rule::danotw,
            "Expected `danotw` rule when building DANOTW statement, found: {:?}",
            pair.as_rule()
        );

        let mut inner = pair.into_inner();

        let weights = Expr::from_rule(inner.next().ok_or_else(|| anyhow::anyhow!("Missing weights in DANOTW"))?)
            .map_err(|e| e)?
            .ok_or_else(|| anyhow::anyhow!("Expected weights expression in DANOTW"))?;

        let size = Expr::from_rule(inner.next().ok_or_else(|| anyhow::anyhow!("Missing size in DANOTW"))?)
            .map_err(|e| e)?
            .ok_or_else(|| anyhow::anyhow!("Expected size expression in DANOTW"))?;

        Ok(Some(DanotwStatement { weights, size }))
    }
}

impl TranspileableStatement for DanotwStatement {
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

impl Transpile for DanotwStatement {
    fn transpile(
        &self,
        _context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let serialization =
            "anyhow::bail!(\"DANOTW is not yet implemented: weighted monomial ordering requires rearchitecting init_taylor\");".to_string();

        Ok(TranspilationOutput {
            serialization,
            ..Default::default()
        })
    }
}
