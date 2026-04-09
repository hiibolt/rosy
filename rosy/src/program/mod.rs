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

use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use crate::{
    ast::{CosyParser, FromRule, Rule},
    program::statements::{SourceLocation, Statement},
    resolve::*,
    transpile::*,
};
use anyhow::{Context, Error, Result, bail};
use pest::Parser;

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
        Program::from_rule_with_includes(pair, None, &mut HashSet::new())
    }
}

impl Program {
    /// Parse a program, recursively resolving INCLUDE statements at the AST level.
    ///
    /// Each included file must be a complete `BEGIN; ... END;` program.
    /// Its statements are spliced into the parent at the INCLUDE site.
    pub fn from_rule_with_includes(
        pair: pest::iterators::Pair<Rule>,
        source_path: Option<&Path>,
        visited: &mut HashSet<PathBuf>,
    ) -> Result<Option<Program>> {
        let mut statements = Vec::new();

        for stmt in pair.into_inner() {
            if stmt.as_rule() == Rule::include_stmt {
                // Extract the path from the string literal inside `include_stmt`
                let include_path = Self::extract_include_path(&stmt)?;

                // Resolve relative to the including file's directory
                let base_dir = source_path.and_then(|p| p.parent());
                let resolved = if Path::new(&include_path).is_absolute() {
                    PathBuf::from(&include_path)
                } else {
                    let base = base_dir.ok_or_else(|| {
                        anyhow::anyhow!(
                            "Cannot resolve relative INCLUDE '{}' — source file path is unknown \
                             (hint: save the file to disk first)",
                            include_path,
                        )
                    })?;
                    base.join(&include_path)
                };

                let canonical = std::fs::canonicalize(&resolved).with_context(|| {
                    format!("Failed to resolve INCLUDE path '{}'", include_path)
                })?;

                // Circular include detection
                if visited.contains(&canonical) {
                    let chain: Vec<String> = visited.iter().map(|p| p.display().to_string()).collect();
                    bail!(
                        "Circular INCLUDE detected: {} → {}",
                        chain.join(" → "),
                        canonical.display()
                    );
                }

                // Read and parse the included file as a full program
                let included_source = std::fs::read_to_string(&canonical).with_context(|| {
                    format!("Failed to read INCLUDE file '{}'", resolved.display())
                })?;

                let mut pairs = CosyParser::parse(Rule::program, &included_source)
                    .with_context(|| format!("Failed to parse INCLUDE file '{}'", canonical.display()))?;

                let program_pair = pairs.next()
                    .ok_or_else(|| anyhow::anyhow!("Empty parse result for '{}'", canonical.display()))?;

                // Recurse
                visited.insert(canonical.clone());
                let included_program = Program::from_rule_with_includes(
                    program_pair,
                    Some(&canonical),
                    visited,
                )?;
                visited.remove(&canonical);

                // Splice the included statements, stamping file origin
                if let Some(prog) = included_program {
                    for mut s in prog.statements {
                        if s.source_location.file.is_none() {
                            s.source_location.file = Some(canonical.clone());
                        }
                        statements.push(s);
                    }
                }
            } else {
                let pair_input = stmt.as_str();
                if let Some(statement) = Statement::from_rule(stmt)
                    .with_context(|| format!("Failed to build statement from:\n{}", pair_input))?
                {
                    statements.push(statement);
                }
            }
        }

        Ok(Some(Program { statements }))
    }

    /// Extract the file path string from an `include_stmt` pair.
    fn extract_include_path(pair: &pest::iterators::Pair<Rule>) -> Result<String> {
        // include_stmt = { ^"INCLUDE" ~ string ~ semicolon }
        // string = { new_string | old_string }
        // new_string = @{ "\"" ~ ... ~ "\"" }
        // old_string = @{ "\'" ~ ... ~ "\'" }
        let string_pair = pair.clone().into_inner()
            .find(|p| p.as_rule() == Rule::string)
            .ok_or_else(|| anyhow::anyhow!("INCLUDE statement missing path string"))?;

        let inner = string_pair.into_inner().next()
            .ok_or_else(|| anyhow::anyhow!("INCLUDE path string is empty"))?;

        let raw = inner.as_str();
        // Strip surrounding quotes (first and last char)
        let path = &raw[1..raw.len() - 1];

        // For old_string (single-quoted), unescape ''  → '
        let path = if inner.as_rule() == Rule::old_string {
            path.replace("''", "'")
        } else {
            path.to_string()
        };

        Ok(path)
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
