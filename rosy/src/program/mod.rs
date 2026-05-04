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

/// Tracks INCLUDE resolution across one compilation unit.
///
/// Two distinct concerns share this struct because they share lookup paths:
///
/// * `in_progress` — files currently being parsed up the recursion stack.
///   A repeated INCLUDE of an in-progress file is a true cycle (A→B→A) and
///   surfaces as a `Circular INCLUDE detected` error.
///
/// * `completed` — files that have been fully resolved at least once. A
///   repeated INCLUDE of a completed file is silently skipped (no-op),
///   which mirrors Rust's `mod foo;` and Python's `import foo` semantics:
///   declarations inside the included file enter the program exactly once,
///   even when several leaf files all `INCLUDE` the same library.
///
/// Without `completed`, a library file that shares a header (e.g.
/// `libcosy/helpers/math.rosy` declaring `INCLUDE '../globals.rosy';` so
/// it stands alone for LSP analysis) would re-emit every VARIABLE in
/// globals when transitively pulled through `INCLUDE 'libcosy';`.
#[derive(Debug, Default)]
pub struct IncludeTracker {
    in_progress: HashSet<PathBuf>,
    completed: HashSet<PathBuf>,
}

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
        Program::from_rule_with_includes(pair, None, &mut IncludeTracker::default())
    }
}

impl Program {
    /// Parse a program, recursively resolving INCLUDE statements at the AST level.
    ///
    /// Each included file must be a complete `BEGIN; ... END;` program.
    /// Its statements are spliced into the parent at the INCLUDE site.
    /// Repeated INCLUDEs of the same file (across different parents) are
    /// idempotent — declarations enter the program exactly once.
    pub fn from_rule_with_includes(
        pair: pest::iterators::Pair<Rule>,
        source_path: Option<&Path>,
        tracker: &mut IncludeTracker,
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

                // Resolve to a concrete `mod.rosy` file:
                //   (1) `resolved` is a regular file        → use it (current behavior)
                //   (2) `resolved` is a directory           → look for `<dir>/mod.rosy`
                //   (3) `resolved` doesn't exist            → still try `<resolved>/mod.rosy`
                //                                             (so `INCLUDE 'libcosy';` works
                //                                              before any `libcosy.rosy` exists)
                let canonical = match std::fs::canonicalize(&resolved) {
                    Ok(p) if p.is_file() => p,
                    Ok(p) if p.is_dir() => {
                        let mod_path = p.join("mod.rosy");
                        std::fs::canonicalize(&mod_path).with_context(|| {
                            format!(
                                "INCLUDE '{}' resolved to directory '{}' but no 'mod.rosy' was found inside.\n\
                                 Hint: create '{}/mod.rosy' or include a specific .rosy file.",
                                include_path,
                                p.display(),
                                p.display(),
                            )
                        })?
                    }
                    Ok(p) => bail!(
                        "INCLUDE '{}' resolved to '{}' which is neither a regular file nor a directory",
                        include_path,
                        p.display(),
                    ),
                    Err(_) => {
                        let mod_path = resolved.join("mod.rosy");
                        std::fs::canonicalize(&mod_path).with_context(|| {
                            format!(
                                "Failed to resolve INCLUDE path '{}' — tried '{}' (file) and '{}/mod.rosy' (directory module)",
                                include_path,
                                resolved.display(),
                                resolved.display(),
                            )
                        })?
                    }
                };

                // Idempotency: a file that has already been fully parsed once
                // contributes its declarations exactly once. Subsequent INCLUDEs
                // are silent no-ops, so library files can safely INCLUDE their
                // own dependencies without producing duplicate VARIABLEs when
                // the program also INCLUDEs those dependencies via another path.
                if tracker.completed.contains(&canonical) {
                    continue;
                }

                // True cycle detection: only the active recursion stack counts.
                if tracker.in_progress.contains(&canonical) {
                    let chain: Vec<String> = tracker
                        .in_progress
                        .iter()
                        .map(|p| p.display().to_string())
                        .collect();
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
                tracker.in_progress.insert(canonical.clone());
                let included_program = Program::from_rule_with_includes(
                    program_pair,
                    Some(&canonical),
                    tracker,
                )?;
                tracker.in_progress.remove(&canonical);
                tracker.completed.insert(canonical.clone());

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
#[cfg(test)]
mod include_resolution_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn parse_and_resolve(source: &str, source_path: &Path) -> Result<Program> {
        let mut pairs = CosyParser::parse(Rule::program, source)
            .map_err(|e| anyhow::anyhow!("parse error: {e}"))?;
        let pair = pairs
            .next()
            .ok_or_else(|| anyhow::anyhow!("empty parse"))?;
        Program::from_rule_with_includes(pair, Some(source_path), &mut IncludeTracker::default())?
            .ok_or_else(|| anyhow::anyhow!("from_rule_with_includes returned None"))
    }

    fn write(dir: &Path, name: &str, contents: &str) -> PathBuf {
        let path = dir.join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&path, contents).unwrap();
        path
    }

    /// Baseline: existing behavior — INCLUDE points at a literal `.rosy` file.
    #[test]
    fn include_resolves_literal_file() {
        let tmp = TempDir::new().unwrap();
        write(tmp.path(), "helper.rosy", "BEGIN;\nEND;\n");
        let main = write(tmp.path(), "main.rosy", "BEGIN;\nINCLUDE 'helper.rosy';\nEND;\n");
        let src = fs::read_to_string(&main).unwrap();
        parse_and_resolve(&src, &main).expect("literal-file include should succeed");
    }

    /// New behavior: INCLUDE points at a directory; we look for `<dir>/mod.rosy`.
    #[test]
    fn include_resolves_directory_with_modrosy() {
        let tmp = TempDir::new().unwrap();
        write(tmp.path(), "libcosy/mod.rosy", "BEGIN;\nEND;\n");
        let main = write(tmp.path(), "main.rosy", "BEGIN;\nINCLUDE 'libcosy';\nEND;\n");
        let src = fs::read_to_string(&main).unwrap();
        parse_and_resolve(&src, &main).expect("directory include should resolve to mod.rosy");
    }

    /// Directory exists but has no `mod.rosy` — the error must mention both attempts.
    #[test]
    fn include_directory_without_modrosy_errors() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("libcosy")).unwrap();
        let main = write(tmp.path(), "main.rosy", "BEGIN;\nINCLUDE 'libcosy';\nEND;\n");
        let src = fs::read_to_string(&main).unwrap();
        let err = parse_and_resolve(&src, &main).unwrap_err();
        let chain: Vec<String> = err.chain().map(|e| e.to_string()).collect();
        let joined = chain.join(" :: ");
        assert!(
            joined.contains("mod.rosy"),
            "error chain should mention 'mod.rosy': {joined}"
        );
    }

    /// Neither a file nor a directory at the path — error names both attempted paths.
    #[test]
    fn include_nonexistent_errors() {
        let tmp = TempDir::new().unwrap();
        let main = write(tmp.path(), "main.rosy", "BEGIN;\nINCLUDE 'nope';\nEND;\n");
        let src = fs::read_to_string(&main).unwrap();
        let err = parse_and_resolve(&src, &main).unwrap_err();
        let joined: String = err.chain().map(|e| e.to_string()).collect::<Vec<_>>().join(" :: ");
        assert!(
            joined.contains("nope"),
            "error should mention the missing 'nope': {joined}"
        );
        assert!(
            joined.contains("mod.rosy") || joined.contains("directory module"),
            "error should mention the directory-fallback attempt: {joined}"
        );
    }

    /// Relative paths inside a directory's `mod.rosy` resolve relative to that file's dir,
    /// so a parent `mod.rosy` can `INCLUDE 'sibling';` to load `sibling/mod.rosy`.
    #[test]
    fn include_relative_path_through_directory() {
        let tmp = TempDir::new().unwrap();
        write(tmp.path(), "libcosy/physics/mod.rosy", "BEGIN;\nEND;\n");
        write(
            tmp.path(),
            "libcosy/mod.rosy",
            "BEGIN;\nINCLUDE 'physics';\nEND;\n",
        );
        let main = write(tmp.path(), "main.rosy", "BEGIN;\nINCLUDE 'libcosy';\nEND;\n");
        let src = fs::read_to_string(&main).unwrap();
        parse_and_resolve(&src, &main)
            .expect("nested directory include should resolve transitively");
    }

    /// Including the same file twice (via different INCLUDE chains) is a
    /// silent no-op the second time — declarations enter the program once.
    /// This protects libraries that INCLUDE their own dependencies for
    /// standalone analysis.
    #[test]
    fn include_idempotent_double_include() {
        let tmp = TempDir::new().unwrap();
        // header.rosy declares a single global; it must NOT be redeclared
        // when reached via two different INCLUDE chains.
        write(tmp.path(), "header.rosy", "BEGIN;\nVARIABLE (RE) SHARED;\nEND;\n");
        // a.rosy and b.rosy both include header.rosy …
        write(tmp.path(), "a.rosy", "BEGIN;\nINCLUDE 'header.rosy';\nEND;\n");
        write(tmp.path(), "b.rosy", "BEGIN;\nINCLUDE 'header.rosy';\nEND;\n");
        // … and main.rosy includes both, so header.rosy would otherwise be
        // spliced twice → "VARIABLE 'SHARED' already defined".
        let main = write(
            tmp.path(),
            "main.rosy",
            "BEGIN;\nINCLUDE 'a.rosy';\nINCLUDE 'b.rosy';\nEND;\n",
        );
        let src = fs::read_to_string(&main).unwrap();
        let prog = parse_and_resolve(&src, &main)
            .expect("idempotent INCLUDE should silently skip the second visit");
        // header.rosy contributes exactly one statement (`VARIABLE SHARED`).
        // a.rosy and b.rosy each contribute zero of their own. Without
        // idempotency the count would be 2 (one per chain).
        assert_eq!(
            prog.statements.len(),
            1,
            "expected exactly one statement after idempotent INCLUDE, got {}",
            prog.statements.len()
        );
    }

    /// Circular detection still fires when the cycle goes through directory modules.
    #[test]
    fn include_circular_through_directories() {
        let tmp = TempDir::new().unwrap();
        write(
            tmp.path(),
            "a/mod.rosy",
            "BEGIN;\nINCLUDE '../b';\nEND;\n",
        );
        write(
            tmp.path(),
            "b/mod.rosy",
            "BEGIN;\nINCLUDE '../a';\nEND;\n",
        );
        let main = write(tmp.path(), "main.rosy", "BEGIN;\nINCLUDE 'a';\nEND;\n");
        let src = fs::read_to_string(&main).unwrap();
        let err = parse_and_resolve(&src, &main).unwrap_err();
        let joined: String = err.chain().map(|e| e.to_string()).collect::<Vec<_>>().join(" :: ");
        assert!(
            joined.contains("Circular INCLUDE"),
            "error chain should report circular INCLUDE: {joined}"
        );
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
