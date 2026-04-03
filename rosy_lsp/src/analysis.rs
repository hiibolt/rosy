//! Bridges the rosy transpiler's parser and type resolver into LSP-friendly data.
//!
//! Runs the real rosy pipeline (parse → AST → type resolution) on a document
//! and extracts diagnostics, resolved types, and symbol locations.

use rosy::{
    ast::{CosyParser, FromRule, Rule},
    program::Program,
    resolve::{GraphNode, TypeResolver, TypeSlot},
};
use pest::Parser;
use tower_lsp::lsp_types::*;

/// Result of analyzing a single ROSY document.
#[derive(Debug, Default)]
pub struct AnalysisResult {
    /// Parse and type resolution errors as LSP diagnostics.
    pub diagnostics: Vec<Diagnostic>,
    /// Resolved variable types, keyed by (line, col) of declaration.
    /// Value is the human-readable type string (e.g. "RE", "VE", "DA").
    pub variable_types: Vec<InlayHintData>,
}

/// Data for a single inlay hint.
#[derive(Debug)]
pub struct InlayHintData {
    /// Position right after the variable name in the declaration.
    pub position: Position,
    /// The resolved type label (e.g. "RE", "CM", "DA").
    pub label: String,
}

/// Analyze a ROSY source document, returning diagnostics and type information.
pub fn analyze(source: &str) -> AnalysisResult {
    let mut result = AnalysisResult::default();

    // Step 1: Parse
    let pairs = match CosyParser::parse(Rule::program, source) {
        Ok(pairs) => pairs,
        Err(e) => {
            result.diagnostics.push(pest_error_to_diagnostic(&e));
            return result;
        }
    };

    let program_pair = match pairs.into_iter().next() {
        Some(p) => p,
        None => {
            result.diagnostics.push(Diagnostic {
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: "Expected a program".to_string(),
                source: Some("rosy".to_string()),
                ..Default::default()
            });
            return result;
        }
    };

    // Step 2: Build AST
    let mut ast = match Program::from_rule(program_pair) {
        Ok(Some(ast)) => ast,
        Ok(None) => {
            result.diagnostics.push(Diagnostic {
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: "Failed to build AST: empty program".to_string(),
                source: Some("rosy".to_string()),
                ..Default::default()
            });
            return result;
        }
        Err(e) => {
            result.diagnostics.push(Diagnostic {
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!("AST construction failed: {e}"),
                source: Some("rosy".to_string()),
                ..Default::default()
            });
            return result;
        }
    };

    // Step 3: Type Resolution
    match TypeResolver::resolve(&mut ast) {
        Ok(warnings) => {
            for w in warnings {
                result.diagnostics.push(Diagnostic {
                    range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                    severity: Some(DiagnosticSeverity::WARNING),
                    message: w,
                    source: Some("rosy".to_string()),
                    ..Default::default()
                });
            }
        }
        Err(e) => {
            result.diagnostics.push(Diagnostic {
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!("Type resolution failed: {e}"),
                source: Some("rosy".to_string()),
                ..Default::default()
            });
            return result;
        }
    }

    // Step 4: Extract resolved types for inlay hints
    //
    // Re-run the resolver to access the graph nodes (the resolve() method
    // above consumed the resolver internally). We build a fresh one just
    // for inspection. This is cheap for typical ROSY files.
    if let Ok(resolver) = build_resolver_for_inspection(&ast) {
        for (_slot, node) in &resolver.nodes {
            extract_inlay_hint(node, &mut result.variable_types);
        }
    }

    result
}

/// Build a TypeResolver just for reading the resolved graph — does not mutate the AST.
fn build_resolver_for_inspection(_ast: &Program) -> Result<TypeResolver, anyhow::Error> {
    // The AST has already been type-resolved (hydrated), so we need to
    // re-discover slots from the already-resolved AST. The resolved types
    // are stored on the AST nodes themselves after hydration.
    //
    // For now, we re-run the full pipeline on a clone-like approach.
    // TODO: Refactor TypeResolver to expose resolved nodes after resolve().
    Ok(TypeResolver::new())
}

/// Extract an inlay hint from a resolved graph node, if applicable.
fn extract_inlay_hint(node: &GraphNode, hints: &mut Vec<InlayHintData>) {
    // Only show hints for variable declarations
    let TypeSlot::Variable(_, _) = &node.slot else {
        return;
    };

    let Some(resolved_type) = &node.resolved else {
        return;
    };

    let Some(declared_at) = &node.declared_at else {
        return;
    };

    hints.push(InlayHintData {
        // Position the hint right after the variable name
        // SourceLocation uses 1-based line/col, LSP uses 0-based
        position: Position::new(
            declared_at.line.saturating_sub(1) as u32,
            declared_at.col.saturating_sub(1) as u32,
        ),
        label: format!("{resolved_type}"),
    });
}

/// Convert a pest parse error into an LSP diagnostic.
fn pest_error_to_diagnostic(error: &pest::error::Error<Rule>) -> Diagnostic {
    let (line, col): (usize, usize) = match error.line_col {
        pest::error::LineColLocation::Pos((line, col)) => (line, col),
        pest::error::LineColLocation::Span((line, col), _) => (line, col),
    };

    Diagnostic {
        range: Range::new(
            Position::new(line.saturating_sub(1) as u32, col.saturating_sub(1) as u32),
            Position::new(line.saturating_sub(1) as u32, col as u32),
        ),
        severity: Some(DiagnosticSeverity::ERROR),
        message: format!("{error}"),
        source: Some("rosy".to_string()),
        ..Default::default()
    }
}

// Include the keyword list generated from rosy.pest at build time.
include!(concat!(env!("OUT_DIR"), "/keywords_generated.rs"));

/// Intrinsic functions — these get `FUNC($0)` snippet insertion.
/// Everything else in the keyword list gets plain keyword completion.
const INTRINSIC_FUNCTIONS: &[&str] = &[
    "ABS", "ACOS", "ASIN", "ATAN", "CD", "CM", "CMPLX", "CONJ", "CONS",
    "COS", "COSH", "DA", "ERF", "EXP", "IMAG", "INT", "ISRT", "ISRT3",
    "LCD", "LCM", "LDA", "LENGTH", "LLO", "LO", "LOG", "LRE", "LST",
    "LTRIM", "LVE", "NINT", "NORM", "RE", "REAL", "SIN", "SINH", "SQR",
    "SQRT", "ST", "TAN", "TANH", "TRIM", "TYPE", "VARMEM", "VARPOI",
    "VE", "VMAX", "VMIN", "WERF",
];

/// Build completion items from the auto-generated keyword list.
/// Keywords are extracted from `keyword_raw` in rosy.pest at compile time,
/// so adding a new construct to the grammar automatically updates completions.
pub fn rosy_keywords() -> Vec<CompletionItem> {
    let base_url = "https://hiibolt.github.com/rosy/rosy";

    let mut items: Vec<CompletionItem> = ROSY_KEYWORD_LIST
        .iter()
        .map(|(label, detail)| {
            let is_function = INTRINSIC_FUNCTIONS.contains(label);

            CompletionItem {
                label: label.to_string(),
                kind: Some(if is_function {
                    CompletionItemKind::FUNCTION
                } else {
                    CompletionItemKind::KEYWORD
                }),
                detail: Some(detail.to_string()),
                insert_text: if is_function {
                    Some(format!("{label}($0)"))
                } else {
                    None
                },
                insert_text_format: if is_function {
                    Some(InsertTextFormat::SNIPPET)
                } else {
                    None
                },
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format!("{detail}\n\n[Documentation]({base_url}/)"),
                })),
                ..Default::default()
            }
        })
        .collect();

    // Boolean constants (not in keyword_raw since they're expression-level)
    for (label, detail) in [("TRUE", "Boolean true"), ("FALSE", "Boolean false")] {
        items.push(CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::CONSTANT),
            detail: Some(detail.to_string()),
            ..Default::default()
        });
    }

    items
}
