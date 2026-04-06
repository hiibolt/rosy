//! # Stage 0: Preprocessor
//!
//! Resolves `INCLUDE 'filename' ;` directives by textual substitution
//! **before** the source is parsed. This mirrors the COSY INFINITY
//! preprocessor behaviour, with one deliberate change: relative paths
//! are resolved from the **including file's directory** rather than the
//! working directory.
//!
//! ## Guarantees
//!
//! - String literals (`'...'`, `"..."`) and comments (`{ ... }`) are
//!   never scanned for INCLUDE directives.
//! - Circular includes are detected and reported with the full chain.
//! - A [`SourceMap`] is returned alongside the expanded text so that
//!   downstream error messages can be remapped to the correct file and
//!   line.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

// ── Public API ──────────────────────────────────────────────────────

/// Result of preprocessing: the fully-expanded source text and a map
/// from expanded-line → (origin file, original line).
#[derive(Debug, Clone)]
pub struct PreprocessedSource {
    /// Fully expanded source with all INCLUDEs resolved.
    pub text: String,
    /// One entry per line in `text`. Each entry is
    /// `(source_file, 1-based line number in that file)`.
    pub source_map: SourceMap,
}

/// Maps lines in preprocessed text back to their origin.
#[derive(Debug, Clone, Default)]
pub struct SourceMap {
    /// Index *i* describes expanded line *i + 1*.
    entries: Vec<SourceOrigin>,
}

#[derive(Debug, Clone)]
pub struct SourceOrigin {
    /// Canonical path of the file this line came from.
    pub file: PathBuf,
    /// 1-based line number in that file.
    pub line: usize,
}

impl SourceMap {
    /// Look up the origin of a 1-based line in the expanded source.
    pub fn origin(&self, expanded_line: usize) -> Option<&SourceOrigin> {
        self.entries.get(expanded_line.wrapping_sub(1))
    }

    /// Format a human-readable location string for a 1-based expanded line.
    pub fn display_location(&self, expanded_line: usize) -> String {
        match self.origin(expanded_line) {
            Some(o) => format!("{}:{}", o.file.display(), o.line),
            None => format!("<unknown>:{expanded_line}"),
        }
    }
}

/// Preprocess a Rosy source string, resolving all INCLUDE directives.
///
/// `source_path` is the path of the file being preprocessed. If `None`
/// (e.g. stdin or LSP buffer), INCLUDE directives that use relative
/// paths will fail with a clear error.
pub fn preprocess(source: &str, source_path: Option<&Path>) -> Result<PreprocessedSource> {
    let mut include_stack: Vec<PathBuf> = Vec::new();
    let mut visited: HashSet<PathBuf> = HashSet::new();

    let canonical = source_path
        .map(|p| {
            std::fs::canonicalize(p)
                .unwrap_or_else(|_| p.to_path_buf())
        });

    if let Some(ref p) = canonical {
        include_stack.push(p.clone());
        visited.insert(p.clone());
    }

    let mut text = String::new();
    let mut source_map = SourceMap::default();

    let source_file = canonical
        .clone()
        .unwrap_or_else(|| PathBuf::from("<input>"));

    preprocess_inner(
        source,
        &source_file,
        source_path.and_then(|p| p.parent()),
        &mut include_stack,
        &mut visited,
        &mut text,
        &mut source_map,
    )?;

    Ok(PreprocessedSource { text, source_map })
}

// ── Internal recursive worker ───────────────────────────────────────

fn preprocess_inner(
    source: &str,
    source_file: &Path,
    base_dir: Option<&Path>,
    include_stack: &mut Vec<PathBuf>,
    visited: &mut HashSet<PathBuf>,
    out: &mut String,
    source_map: &mut SourceMap,
) -> Result<()> {
    let lines: Vec<&str> = source.lines().collect();
    let mut line_idx = 0;

    while line_idx < lines.len() {
        let line = lines[line_idx];

        if let Some(include_path) = try_parse_include(line) {
            // Resolve the path relative to the including file's directory
            let resolved = resolve_include_path(&include_path, base_dir, source_file, line_idx + 1)?;

            let canonical = std::fs::canonicalize(&resolved).with_context(|| {
                format!(
                    "{}:{}: failed to resolve INCLUDE path '{}'",
                    source_file.display(),
                    line_idx + 1,
                    include_path
                )
            })?;

            // Circular include detection
            if visited.contains(&canonical) {
                let chain: Vec<String> = include_stack
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect();
                bail!(
                    "{}:{}: circular INCLUDE detected: {} → {}",
                    source_file.display(),
                    line_idx + 1,
                    chain.join(" → "),
                    canonical.display()
                );
            }

            // Read the included file
            let included_source = std::fs::read_to_string(&canonical).with_context(|| {
                format!(
                    "{}:{}: failed to read INCLUDE file '{}'",
                    source_file.display(),
                    line_idx + 1,
                    resolved.display()
                )
            })?;

            // Recurse
            include_stack.push(canonical.clone());
            visited.insert(canonical.clone());

            let included_base_dir = canonical.parent();
            preprocess_inner(
                &included_source,
                &canonical,
                included_base_dir,
                include_stack,
                visited,
                out,
                source_map,
            )?;

            visited.remove(&canonical);
            include_stack.pop();
        } else {
            // Normal line — emit as-is
            out.push_str(line);
            out.push('\n');
            source_map.entries.push(SourceOrigin {
                file: source_file.to_path_buf(),
                line: line_idx + 1,
            });
        }

        line_idx += 1;
    }

    Ok(())
}

// ── INCLUDE line detection ──────────────────────────────────────────

/// Try to parse an INCLUDE directive from a single line.
///
/// Returns `Some(path_string)` if the line is an INCLUDE directive
/// outside of any string or comment context. Returns `None` otherwise.
///
/// Recognised forms:
/// ```text
///   INCLUDE 'file.rosy' ;
///   INCLUDE "file.rosy" ;
///   include 'file.rosy';     { case-insensitive, flexible whitespace }
/// ```
fn try_parse_include(line: &str) -> Option<String> {
    // State machine to skip through the line, respecting strings and comments
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    // We need to find INCLUDE at the "top level" — not inside a string or comment.
    // Since INCLUDE must be a statement on its own line (or at least the dominant
    // construct), we scan for the keyword while tracking nesting.

    loop {
        // Skip whitespace
        while i < len && chars[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= len {
            return None;
        }

        // Check for comment — skip entirely
        if chars[i] == '{' {
            let mut depth = 1;
            i += 1;
            while i < len && depth > 0 {
                if chars[i] == '{' {
                    depth += 1;
                } else if chars[i] == '}' {
                    depth -= 1;
                }
                i += 1;
            }
            continue;
        }

        // Check for string literal — skip entirely
        if chars[i] == '\'' || chars[i] == '"' {
            let quote = chars[i];
            i += 1;
            while i < len {
                if chars[i] == quote {
                    if quote == '\'' && i + 1 < len && chars[i + 1] == '\'' {
                        // Escaped single quote ''
                        i += 2;
                        continue;
                    }
                    break;
                }
                i += 1;
            }
            // If the line starts with a string, it's not an INCLUDE
            return None;
        }

        // Check if we're looking at "INCLUDE" (case-insensitive)
        let remaining = &line[line.char_indices().nth(i).map(|(b, _)| b)?..];
        if remaining.len() >= 7
            && remaining[..7].eq_ignore_ascii_case("INCLUDE")
            && (remaining.len() == 7
                || !remaining.as_bytes()[7].is_ascii_alphanumeric()
                && remaining.as_bytes()[7] != b'_')
        {
            // Found INCLUDE keyword at top level — now extract the path
            return extract_include_path(&remaining[7..]);
        }

        // Not INCLUDE — this line has some other content at top level
        return None;
    }
}

/// Given the text after the INCLUDE keyword, extract the quoted path.
/// E.g. from `  'myfile.rosy' ;` extracts `myfile.rosy`.
fn extract_include_path(after_keyword: &str) -> Option<String> {
    let trimmed = after_keyword.trim_start();

    let (quote, rest) = {
        let mut chars = trimmed.chars();
        let q = chars.next()?;
        if q != '\'' && q != '"' {
            return None;
        }
        (q, chars.as_str())
    };

    // Find the closing quote
    let mut path = String::new();
    let mut chars = rest.chars();
    loop {
        let c = chars.next()?;
        if c == quote {
            if quote == '\'' {
                // Check for escaped ''
                if chars.as_str().starts_with('\'') {
                    path.push('\'');
                    chars.next();
                    continue;
                }
            }
            break;
        }
        path.push(c);
    }

    // The rest should be optional whitespace and a semicolon
    let remainder = chars.as_str().trim();
    if !remainder.is_empty() && !remainder.starts_with(';') {
        return None; // Trailing junk — not a valid INCLUDE
    }

    if path.is_empty() {
        return None;
    }

    Some(path)
}

// ── Path resolution ─────────────────────────────────────────────────

fn resolve_include_path(
    include_path: &str,
    base_dir: Option<&Path>,
    source_file: &Path,
    line_num: usize,
) -> Result<PathBuf> {
    let path = Path::new(include_path);

    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    let base = base_dir.ok_or_else(|| {
        anyhow::anyhow!(
            "{}:{}: cannot resolve relative INCLUDE '{}' — source file path is unknown \
             (hint: save the file to disk first)",
            source_file.display(),
            line_num,
            include_path,
        )
    })?;

    Ok(base.join(path))
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_include_detection() {
        assert_eq!(
            try_parse_include("  INCLUDE 'foo.rosy' ;"),
            Some("foo.rosy".to_string())
        );
        assert_eq!(
            try_parse_include("include \"bar.rosy\";"),
            Some("bar.rosy".to_string())
        );
        assert_eq!(
            try_parse_include("  Include 'baz.rosy'  ;  "),
            Some("baz.rosy".to_string())
        );
    }

    #[test]
    fn include_without_semicolon() {
        // INCLUDE without semicolon should still work (semicolon is optional trailing)
        assert_eq!(
            try_parse_include("INCLUDE 'foo.rosy'"),
            Some("foo.rosy".to_string())
        );
    }

    #[test]
    fn include_in_comment_ignored() {
        assert_eq!(try_parse_include("{ INCLUDE 'foo.rosy' ; }"), None);
        assert_eq!(
            try_parse_include("{ comment } INCLUDE 'foo.rosy' ;"),
            Some("foo.rosy".to_string())
        );
    }

    #[test]
    fn include_in_string_ignored() {
        // A line that starts with a string is never an INCLUDE
        assert_eq!(try_parse_include("'INCLUDE something' ;"), None);
    }

    #[test]
    fn not_include_keyword() {
        // INCLUDES (with trailing S) should not match
        assert_eq!(try_parse_include("INCLUDES 'foo.rosy' ;"), None);
        // INCLUDE_FILE should not match
        assert_eq!(try_parse_include("INCLUDE_FILE 'foo.rosy' ;"), None);
    }

    #[test]
    fn escaped_quotes_in_path() {
        assert_eq!(
            try_parse_include("INCLUDE 'it''s.rosy' ;"),
            Some("it's.rosy".to_string())
        );
    }

    #[test]
    fn no_include() {
        assert_eq!(try_parse_include("VARIABLE (RE) X;"), None);
        assert_eq!(try_parse_include("WRITE 6 'hello';"), None);
        assert_eq!(try_parse_include(""), None);
    }

    #[test]
    fn preprocess_no_includes() {
        let source = "BEGIN;\n  WRITE 6 'hi';\nEND;\n";
        let result = preprocess(source, None).unwrap();
        assert_eq!(result.text, source);
        assert_eq!(result.source_map.entries.len(), 3);
    }

    #[test]
    fn preprocess_circular_detection() {
        use std::io::Write;
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.rosy");
        let b = dir.path().join("b.rosy");
        std::fs::File::create(&a)
            .unwrap()
            .write_all(b"INCLUDE 'b.rosy' ;\n")
            .unwrap();
        std::fs::File::create(&b)
            .unwrap()
            .write_all(b"INCLUDE 'a.rosy' ;\n")
            .unwrap();
        let source = std::fs::read_to_string(&a).unwrap();
        let err = preprocess(&source, Some(&a)).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("circular INCLUDE"), "expected circular error, got: {msg}");
    }

    #[test]
    fn preprocess_nested_includes() {
        use std::io::Write;
        let dir = tempfile::tempdir().unwrap();
        let main_file = dir.path().join("main.rosy");
        let helper = dir.path().join("helper.rosy");

        std::fs::File::create(&helper)
            .unwrap()
            .write_all(b"WRITE 6 'from helper';\n")
            .unwrap();
        std::fs::File::create(&main_file)
            .unwrap()
            .write_all(b"BEGIN;\nINCLUDE 'helper.rosy' ;\nEND;\n")
            .unwrap();

        let source = std::fs::read_to_string(&main_file).unwrap();
        let result = preprocess(&source, Some(&main_file)).unwrap();

        assert!(result.text.contains("from helper"));
        assert!(!result.text.contains("INCLUDE"));
        // Line 1 = BEGIN from main, line 2 = WRITE from helper, line 3 = END from main
        assert_eq!(result.source_map.entries.len(), 3);
        assert_eq!(result.source_map.entries[1].line, 1); // line 1 of helper
    }

    #[test]
    fn source_map_tracks_origins() {
        use std::io::Write;
        let dir = tempfile::tempdir().unwrap();
        let main_file = dir.path().join("main.rosy");
        let inc = dir.path().join("inc.rosy");

        std::fs::File::create(&inc)
            .unwrap()
            .write_all(b"line1_inc\nline2_inc\n")
            .unwrap();
        std::fs::File::create(&main_file)
            .unwrap()
            .write_all(b"line1_main\nINCLUDE 'inc.rosy' ;\nline3_main\n")
            .unwrap();

        let source = std::fs::read_to_string(&main_file).unwrap();
        let result = preprocess(&source, Some(&main_file)).unwrap();

        // Expanded: line1_main / line1_inc / line2_inc / line3_main
        assert_eq!(result.source_map.entries.len(), 4);

        let o1 = result.source_map.origin(1).unwrap();
        assert!(o1.file.ends_with("main.rosy"));
        assert_eq!(o1.line, 1);

        let o2 = result.source_map.origin(2).unwrap();
        assert!(o2.file.ends_with("inc.rosy"));
        assert_eq!(o2.line, 1);

        let o3 = result.source_map.origin(3).unwrap();
        assert!(o3.file.ends_with("inc.rosy"));
        assert_eq!(o3.line, 2);

        let o4 = result.source_map.origin(4).unwrap();
        assert!(o4.file.ends_with("main.rosy"));
        assert_eq!(o4.line, 3);
    }
}
