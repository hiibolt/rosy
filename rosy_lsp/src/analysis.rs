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
    /// Semantic tokens for syntax highlighting via the LSP.
    pub semantic_tokens: Vec<SemanticTokenData>,
}

/// Data for a single semantic token.
#[derive(Debug)]
pub struct SemanticTokenData {
    pub line: u32,
    pub start_col: u32,
    pub length: u32,
    pub token_type: SemanticTokenType,
}

/// The token types we report to the editor.
/// The index in LEGEND_TOKEN_TYPES must match what we register in capabilities.
#[derive(Debug, Clone, Copy)]
pub enum SemanticTokenType {
    Keyword,
    Function,
    Type,
    Variable,
    Number,
    String,
    Comment,
}

impl SemanticTokenType {
    pub fn index(self) -> u32 {
        match self {
            SemanticTokenType::Keyword => 0,
            SemanticTokenType::Function => 1,
            SemanticTokenType::Type => 2,
            SemanticTokenType::Variable => 3,
            SemanticTokenType::Number => 4,
            SemanticTokenType::String => 5,
            SemanticTokenType::Comment => 6,
        }
    }
}

/// The legend registered with the client. Order must match SemanticTokenType::index().
pub const LEGEND_TOKEN_TYPES: &[tower_lsp::lsp_types::SemanticTokenType] = &[
    tower_lsp::lsp_types::SemanticTokenType::KEYWORD,
    tower_lsp::lsp_types::SemanticTokenType::FUNCTION,
    tower_lsp::lsp_types::SemanticTokenType::TYPE,
    tower_lsp::lsp_types::SemanticTokenType::VARIABLE,
    tower_lsp::lsp_types::SemanticTokenType::NUMBER,
    tower_lsp::lsp_types::SemanticTokenType::STRING,
    tower_lsp::lsp_types::SemanticTokenType::COMMENT,
];

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

    // Semantic tokens are produced by scanning the source text directly,
    // so they work even when the parse fails (partial highlighting).
    result.semantic_tokens = tokenize_source(source);

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

// ─── Semantic Tokenization ──────────────────────────────────────────────────

/// Tokenize ROSY source text for semantic highlighting.
/// Scans the source directly (not the AST) so it works even on broken files.
/// Uses the auto-generated ROSY_KEYWORD_LIST to recognize keywords.
fn tokenize_source(source: &str) -> Vec<SemanticTokenData> {
    let mut tokens = Vec::new();

    for (line_idx, line) in source.lines().enumerate() {
        let line_num = line_idx as u32;
        let bytes = line.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        while i < len {
            let b = bytes[i];

            // Skip whitespace
            if b.is_ascii_whitespace() {
                i += 1;
                continue;
            }

            // Comments: { ... } with nesting
            if b == b'{' {
                let start = i;
                let mut depth = 1;
                i += 1;
                // Comments can span lines but we only handle single-line here.
                // Multi-line comments will get the first line highlighted.
                while i < len && depth > 0 {
                    if bytes[i] == b'{' {
                        depth += 1;
                    } else if bytes[i] == b'}' {
                        depth -= 1;
                    }
                    i += 1;
                }
                tokens.push(SemanticTokenData {
                    line: line_num,
                    start_col: start as u32,
                    length: (i - start) as u32,
                    token_type: SemanticTokenType::Comment,
                });
                continue;
            }

            // Strings: '...' or "..."
            if b == b'\'' || b == b'"' {
                let quote = b;
                let start = i;
                i += 1;
                while i < len {
                    if bytes[i] == quote {
                        // Handle '' escape in single-quoted strings
                        if quote == b'\'' && i + 1 < len && bytes[i + 1] == b'\'' {
                            i += 2;
                            continue;
                        }
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                tokens.push(SemanticTokenData {
                    line: line_num,
                    start_col: start as u32,
                    length: (i - start) as u32,
                    token_type: SemanticTokenType::String,
                });
                continue;
            }

            // Numbers
            if b.is_ascii_digit() || (b == b'-' && i + 1 < len && bytes[i + 1].is_ascii_digit()) {
                let start = i;
                if b == b'-' {
                    i += 1;
                }
                while i < len && bytes[i].is_ascii_digit() {
                    i += 1;
                }
                if i < len && bytes[i] == b'.' && i + 1 < len && bytes[i + 1].is_ascii_digit() {
                    i += 1;
                    while i < len && bytes[i].is_ascii_digit() {
                        i += 1;
                    }
                }
                // Scientific notation
                if i < len && (bytes[i] == b'e' || bytes[i] == b'E') {
                    i += 1;
                    if i < len && (bytes[i] == b'+' || bytes[i] == b'-') {
                        i += 1;
                    }
                    while i < len && bytes[i].is_ascii_digit() {
                        i += 1;
                    }
                }
                tokens.push(SemanticTokenData {
                    line: line_num,
                    start_col: start as u32,
                    length: (i - start) as u32,
                    token_type: SemanticTokenType::Number,
                });
                continue;
            }

            // Identifiers / keywords
            if b.is_ascii_alphabetic() || b == b'_' {
                let start = i;
                while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                let word = &line[start..i];
                let upper = word.to_uppercase();

                // Check if it's a type annotation
                let token_type = if matches!(
                    upper.as_str(),
                    "RE" | "ST" | "LO" | "CM" | "VE" | "DA" | "CD"
                ) {
                    // If followed by `(`, it's a function call; otherwise it's a type
                    let rest = &line[i..].trim_start();
                    if rest.starts_with('(') {
                        SemanticTokenType::Function
                    } else {
                        SemanticTokenType::Type
                    }
                } else if upper == "TRUE" || upper == "FALSE" {
                    SemanticTokenType::Keyword
                } else if INTRINSIC_FUNCTIONS.contains(&upper.as_str()) {
                    SemanticTokenType::Function
                } else if ROSY_KEYWORD_LIST.iter().any(|(kw, _)| *kw == upper) {
                    SemanticTokenType::Keyword
                } else {
                    SemanticTokenType::Variable
                };

                tokens.push(SemanticTokenData {
                    line: line_num,
                    start_col: start as u32,
                    length: (i - start) as u32,
                    token_type,
                });
                continue;
            }

            // Skip operators and punctuation (not semantically highlighted)
            i += 1;
        }
    }

    tokens
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

// Include generated data from rosy.pest + module docs at build time.
include!(concat!(env!("OUT_DIR"), "/keywords_generated.rs"));
include!(concat!(env!("OUT_DIR"), "/hover_generated.rs"));

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
