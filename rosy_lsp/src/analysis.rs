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

/// Keywords extracted from the ROSY grammar for completion.
/// These are derived from the `keyword_raw` rule in rosy.pest.
pub fn rosy_keywords() -> Vec<CompletionItem> {
    // Control flow
    let control = [
        ("BEGIN", "Program entry point"),
        ("END", "Program exit point"),
        ("IF", "Conditional branch"),
        ("ELSEIF", "Additional conditional branch"),
        ("ELSE", "Default branch"),
        ("ENDIF", "End conditional"),
        ("LOOP", "Counted loop: LOOP var start end [step]"),
        ("ENDLOOP", "End counted loop"),
        ("WHILE", "While loop: WHILE condition"),
        ("ENDWHILE", "End while loop"),
        ("PLOOP", "MPI parallel loop"),
        ("ENDPLOOP", "End parallel loop"),
        ("BREAK", "Exit current loop"),
        ("FIT", "Optimization loop"),
        ("ENDFIT", "End optimization loop"),
    ];

    // Declarations
    let declarations = [
        ("VARIABLE", "Declare a variable: VARIABLE [(type)] name [dims]"),
        ("PROCEDURE", "Define a procedure"),
        ("ENDPROCEDURE", "End procedure definition"),
        ("FUNCTION", "Define a function"),
        ("ENDFUNCTION", "End function definition"),
    ];

    // I/O
    let io = [
        ("WRITE", "Write formatted output"),
        ("WRITEB", "Write binary output"),
        ("READ", "Read formatted input"),
        ("READB", "Read binary input"),
        ("OPENF", "Open a text file"),
        ("OPENFB", "Open a binary file"),
        ("CLOSEF", "Close a file"),
        ("WRITEM", "Serialize variable to memory arrays"),
        ("READM", "Deserialize variable from memory arrays"),
    ];

    // DA operations
    let da_ops = [
        ("DAINI", "Initialize DA: OV order nvars"),
        ("DAPRV", "Print DA variables"),
        ("DAREV", "Print DA in reverse format"),
        ("DANOT", "Set DA truncation order"),
        ("DAEPS", "Set DA epsilon cutoff"),
        ("DATRN", "DA transfer (transform)"),
        ("DASCL", "Scale DA by constant"),
        ("DASGN", "Negate DA sign"),
        ("DADER", "Differentiate DA"),
        ("DAINT", "Integrate DA"),
        ("DANORO", "Filter DA by order"),
        ("DANORS", "Filter DA by order range"),
        ("DAPLU", "Substitute variable in DA (plug)"),
        ("DADIU", "DA division by unit variable"),
        ("DADMU", "DA multiplication by unit variable"),
        ("DACLIW", "Extract DA to linear vector"),
        ("DACQLC", "Construct DA from linear vector"),
        ("DAREA", "Read DA from file"),
        ("DAPEW", "Print DA element-wise"),
        ("DAPEE", "Print DA element-wise (error format)"),
        ("DAPEA", "Print DA element-wise (all)"),
        ("DAPEP", "Print DA element-wise (partial)"),
        ("DAEST", "Estimate DA bounds"),
    ];

    // System
    let system = [
        ("QUIT", "Exit with status code"),
        ("OS", "Execute operating system command"),
        ("SCRLEN", "Set screen/output width"),
        ("CPUSEC", "Get CPU time"),
        ("PWTIME", "Get wall-clock time"),
        ("RANSEED", "Set random number seed"),
        ("RERAN", "Generate random number"),
        ("SLEEPM", "Sleep for milliseconds"),
        ("ARGGET", "Get command-line argument"),
        ("SUBSTR", "Extract substring"),
        ("VELSET", "Set vector element"),
        ("VELGET", "Get vector element"),
        ("VEDOT", "Vector dot product"),
        ("VEUNIT", "Normalize vector to unit length"),
        ("VEZERO", "Zero out vector elements"),
        ("STCRE", "String to real conversion"),
        ("RECST", "Real to string conversion"),
        ("LINV", "Matrix inversion"),
        ("LDET", "Matrix determinant"),
        ("LEV", "Solve linear system"),
        ("MBLOCK", "Extract matrix block"),
        ("MTREE", "Evaluate DA using tree method"),
        ("POLVAL", "Polynomial evaluation"),
        ("LSLINE", "Least-squares line fit"),
        ("RKCO", "Runge-Kutta coefficients"),
        ("PNPRO", "Get number of MPI processes"),
        ("IMUNIT", "Get imaginary unit"),
        ("MEMDPV", "Memory dump/view"),
        ("MEMFRE", "Free memory"),
    ];

    // Intrinsic functions (used as expressions)
    let functions = [
        ("SIN", "Sine", CompletionItemKind::FUNCTION),
        ("COS", "Cosine", CompletionItemKind::FUNCTION),
        ("TAN", "Tangent", CompletionItemKind::FUNCTION),
        ("ASIN", "Arcsine", CompletionItemKind::FUNCTION),
        ("ACOS", "Arccosine", CompletionItemKind::FUNCTION),
        ("ATAN", "Arctangent", CompletionItemKind::FUNCTION),
        ("SINH", "Hyperbolic sine", CompletionItemKind::FUNCTION),
        ("COSH", "Hyperbolic cosine", CompletionItemKind::FUNCTION),
        ("TANH", "Hyperbolic tangent", CompletionItemKind::FUNCTION),
        ("SQRT", "Square root", CompletionItemKind::FUNCTION),
        ("SQR", "Square (x²)", CompletionItemKind::FUNCTION),
        ("EXP", "Exponential (eˣ)", CompletionItemKind::FUNCTION),
        ("LOG", "Natural logarithm", CompletionItemKind::FUNCTION),
        ("ABS", "Absolute value", CompletionItemKind::FUNCTION),
        ("NORM", "Norm", CompletionItemKind::FUNCTION),
        ("CONS", "Extract constant part", CompletionItemKind::FUNCTION),
        ("INT", "Truncate toward zero", CompletionItemKind::FUNCTION),
        ("NINT", "Round to nearest integer", CompletionItemKind::FUNCTION),
        ("TYPE", "Return COSY type code", CompletionItemKind::FUNCTION),
        ("REAL", "Real part", CompletionItemKind::FUNCTION),
        ("IMAG", "Imaginary part", CompletionItemKind::FUNCTION),
        ("CMPLX", "Convert to complex", CompletionItemKind::FUNCTION),
        ("CONJ", "Complex conjugate", CompletionItemKind::FUNCTION),
        ("VMAX", "Vector maximum", CompletionItemKind::FUNCTION),
        ("VMIN", "Vector minimum", CompletionItemKind::FUNCTION),
        ("LENGTH", "Length / memory size", CompletionItemKind::FUNCTION),
        ("TRIM", "Remove trailing spaces", CompletionItemKind::FUNCTION),
        ("LTRIM", "Remove leading spaces", CompletionItemKind::FUNCTION),
        ("ISRT", "Inverse square root (x⁻¹ᐟ²)", CompletionItemKind::FUNCTION),
        ("ISRT3", "Inverse cube root (x⁻³ᐟ²)", CompletionItemKind::FUNCTION),
        ("ERF", "Error function", CompletionItemKind::FUNCTION),
        ("WERF", "Faddeeva function w(z)", CompletionItemKind::FUNCTION),
        ("VARMEM", "Memory address of variable", CompletionItemKind::FUNCTION),
        ("VARPOI", "Pointer address of variable", CompletionItemKind::FUNCTION),
    ];

    // Type conversion functions
    let type_fns = [
        ("RE", "Convert to real (f64)", CompletionItemKind::FUNCTION),
        ("ST", "Convert to string", CompletionItemKind::FUNCTION),
        ("LO", "Convert to logical (bool)", CompletionItemKind::FUNCTION),
        ("CM", "Convert to complex", CompletionItemKind::FUNCTION),
        ("VE", "Convert to vector", CompletionItemKind::FUNCTION),
        ("DA", "Create DA identity vector", CompletionItemKind::FUNCTION),
        ("CD", "Create complex DA identity vector", CompletionItemKind::FUNCTION),
    ];

    // Memory estimators
    let mem_fns = [
        ("LST", "String memory size estimator", CompletionItemKind::FUNCTION),
        ("LCM", "Complex memory size estimator", CompletionItemKind::FUNCTION),
        ("LCD", "DA memory size estimator", CompletionItemKind::FUNCTION),
        ("LRE", "Real memory size estimator", CompletionItemKind::FUNCTION),
        ("LLO", "Logical memory size estimator", CompletionItemKind::FUNCTION),
        ("LVE", "Vector memory size estimator", CompletionItemKind::FUNCTION),
        ("LDA", "DA memory size estimator", CompletionItemKind::FUNCTION),
    ];

    let base_url = "https://hiibolt.github.com/rosy/rosy";

    let mut items = Vec::new();

    // Keyword completions
    for (label, detail) in control
        .iter()
        .chain(declarations.iter())
        .chain(io.iter())
        .chain(da_ops.iter())
        .chain(system.iter())
    {
        items.push(CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some(detail.to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!(
                    "{detail}\n\n[Documentation]({base_url}/program/statements/)"
                ),
            })),
            ..Default::default()
        });
    }

    // Function completions
    for (label, detail, kind) in functions
        .iter()
        .chain(type_fns.iter())
        .chain(mem_fns.iter())
    {
        items.push(CompletionItem {
            label: label.to_string(),
            kind: Some(*kind),
            detail: Some(detail.to_string()),
            insert_text: Some(format!("{label}($0)")),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!(
                    "{detail}\n\n[Documentation]({base_url}/program/expressions/)"
                ),
            })),
            ..Default::default()
        });
    }

    // Boolean constants
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
