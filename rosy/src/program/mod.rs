//! # Rosy Language Reference
//!
//! This is the complete reference for the Rosy programming language. A Rosy
//! program is a `BEGIN; ... END;` block containing [`statements`] that operate
//! on [`expressions`].
//!
//! ## Where to start
//!
//! - **Writing statements** (declarations, loops, I/O, etc.) → **[`statements`]**
//! - **Using expressions** (operators, functions, literals) → **[`expressions`]**
//!
//! Both modules have "Looking for something?" tables that link directly to
//! every language construct.

use std::collections::BTreeSet;

use crate::{
    ast::{FromRule, Rule},
    program::statements::{SourceLocation, Statement},
    resolve::*,
    transpile::*,
};
use anyhow::{Context, Error, Result};

pub mod expressions;
pub mod statements;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
impl TranspileableStatement for Program {
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
impl FromRule for Program {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Program>> {
        let mut statements = Vec::new();

        for stmt in pair.into_inner() {
            let pair_input = stmt.as_str();
            if let Some(statement) = Statement::from_rule(stmt)
                .with_context(|| format!("Failed to build statement from:\n{}", pair_input))?
            {
                statements.push(statement);
            }
        }

        Ok(Some(Program { statements }))
    }
}
impl Transpile for Program {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let mut serialization = Vec::new();
        let mut errors = Vec::new();
        for statement in &self.statements {
            match statement.transpile(context) {
                Ok(output) => {
                    serialization.push(output.serialization);
                }
                Err(stmt_errors) => {
                    for e in stmt_errors {
                        errors.push(e.context("...while transpiling a top-level statement"));
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(TranspilationOutput {
                serialization: serialization.join("\n"),
                requested_variables: BTreeSet::new(),
                ..Default::default()
            })
        } else {
            Err(errors)
        }
    }
}
