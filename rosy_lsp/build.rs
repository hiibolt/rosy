//! Extracts keyword lists, hover documentation, and semantic token mappings
//! from the rosy crate at compile time. Adding a new construct to rosy.pest
//! and its corresponding module automatically updates all LSP features.

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

const BASE_DOC_URL: &str = "https://hiibolt.github.com/rosy/rosy";

fn main() {
    let pest_path = Path::new("../rosy/assets/rosy.pest");
    let program_dir = Path::new("../rosy/src/program");

    println!("cargo:rerun-if-changed={}", pest_path.display());
    println!("cargo:rerun-if-changed={}", program_dir.display());

    let pest_source = fs::read_to_string(pest_path).expect("Failed to read rosy.pest");

    // 1. Extract keywords from `keyword_raw` in rosy.pest
    let keywords = extract_keywords(&pest_source);

    // 2. Scan rosy/src/program/ for module docs and build keyword → doc mapping
    let module_docs = scan_module_docs(program_dir);

    // 3. Build the pest rule name → keyword mapping from grammar rules
    let rule_to_keyword = build_rule_keyword_map(&pest_source);

    // 4. Generate Rust source files
    let out_dir = std::env::var("OUT_DIR").unwrap();
    generate_keywords_file(&out_dir, &keywords, &module_docs);
    generate_hover_file(&out_dir, &module_docs, &rule_to_keyword);
}

// ─── Keyword Extraction ────────────────────────────────────────────────────

fn extract_keywords(source: &str) -> Vec<String> {
    let mut keywords = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("keyword_raw") {
            if let Some(start) = trimmed.find('{') {
                let body = &trimmed[start + 1..];
                let body = body.trim_end_matches('}').trim();

                for part in body.split('|') {
                    if let Some(s) = extract_quoted(part.trim()) {
                        let upper = s.to_uppercase();
                        if !upper.is_empty()
                            && upper != "TRUE"
                            && upper != "FALSE"
                            && !upper.starts_with("ROSY_")
                        {
                            keywords.push(upper);
                        }
                    }
                }
            }
        }
    }

    keywords.sort();
    keywords.dedup();
    keywords
}

fn extract_quoted(s: &str) -> Option<String> {
    let s = s.trim().trim_start_matches('^');
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        Some(s[1..s.len() - 1].to_string())
    } else {
        None
    }
}

// ─── Module Documentation Scanner ──────────────────────────────────────────

/// Information extracted from a module's doc comment.
struct ModuleDoc {
    /// The keyword this module implements (e.g., "SIN", "VARIABLE", "DAINI").
    keyword: String,
    /// First line of the doc comment (the title, e.g., "SIN Function").
    title: String,
    /// Second paragraph — a brief description.
    description: String,
    /// Rustdoc URL path relative to the crate root.
    doc_url: String,
    /// Whether this is a statement or expression module.
    kind: ModuleKind,
}

#[derive(Clone, Copy)]
enum ModuleKind {
    Statement,
    Expression,
}

/// Scan all modules under `program/` and extract their doc comments.
fn scan_module_docs(program_dir: &Path) -> HashMap<String, ModuleDoc> {
    let mut docs = HashMap::new();

    // Walk statements and expressions directories
    for (subdir, kind) in [
        ("statements", ModuleKind::Statement),
        ("expressions", ModuleKind::Expression),
    ] {
        let dir = program_dir.join(subdir);
        if dir.is_dir() {
            scan_modules_recursive(&dir, &dir, kind, &mut docs);
        }
    }

    docs
}

fn scan_modules_recursive(
    base_dir: &Path,
    current_dir: &Path,
    kind: ModuleKind,
    docs: &mut HashMap<String, ModuleDoc>,
) {
    let Ok(entries) = fs::read_dir(current_dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let mod_file = path.join("mod.rs");
            if mod_file.is_file() {
                if let Some(doc) = parse_module_doc(&mod_file, &path, base_dir, kind) {
                    docs.insert(doc.keyword.clone(), doc);
                }
            }
            scan_modules_recursive(base_dir, &path, kind, docs);
        }
    }
}

/// Parse a module's mod.rs to extract its doc comment and derive the keyword.
fn parse_module_doc(
    mod_file: &Path,
    module_dir: &Path,
    base_dir: &Path,
    kind: ModuleKind,
) -> Option<ModuleDoc> {
    let content = fs::read_to_string(mod_file).ok()?;
    let dir_name = module_dir.file_name()?.to_str()?;

    // Map directory name to keyword
    let keyword = dir_name_to_keyword(dir_name)?;

    // Extract //! doc comment lines
    let mut doc_lines = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//!") {
            let comment = trimmed.trim_start_matches("//!").trim();
            doc_lines.push(comment.to_string());
        } else if !trimmed.is_empty() {
            break; // Stop at first non-doc-comment, non-empty line
        }
    }

    if doc_lines.is_empty() {
        return None;
    }

    // Title is the first line (strip leading #)
    let title = doc_lines[0].trim_start_matches('#').trim().to_string();

    // Description is the next non-empty lines until a blank line or ## header
    let mut desc_parts = Vec::new();
    let mut started = false;
    for line in &doc_lines[1..] {
        if line.is_empty() {
            if started {
                break;
            }
            continue;
        }
        if line.starts_with('#') {
            break;
        }
        started = true;
        desc_parts.push(line.as_str());
    }
    let description = desc_parts.join(" ");

    // Build the rustdoc URL path
    let relative = module_dir.strip_prefix(base_dir).ok()?;
    let kind_prefix = match kind {
        ModuleKind::Statement => "program/statements",
        ModuleKind::Expression => "program/expressions",
    };
    let url_path = format!(
        "{BASE_DOC_URL}/{kind_prefix}/{}/",
        relative.display().to_string().replace('\\', "/")
    );

    Some(ModuleDoc {
        keyword,
        title,
        description,
        doc_url: url_path,
        kind,
    })
}

/// Map a module directory name to its ROSY keyword.
fn dir_name_to_keyword(dir_name: &str) -> Option<String> {
    // Special cases where the directory name doesn't match the keyword
    let keyword = match dir_name {
        "var_decl" => "VARIABLE",
        "var_expr" => return None,         // internal, not a keyword
        "variable_identifier" => return None, // internal
        "assign" => return None,           // := is an operator, not a keyword
        "function_call" => return None,    // not a keyword itself
        "procedure_call" => return None,   // not a keyword itself
        "while_loop" => "WHILE",
        "da_init" => "DAINI",
        "break" => "BREAK",
        "if" => "IF",
        "loop" => "LOOP",
        "ploop" => "PLOOP",
        "function" => "FUNCTION",
        "procedure" => "PROCEDURE",
        "quit" => "QUIT",
        "os_call" => "OS",
        "cos_fn" | "cos" => "COS",
        "sin" => "SIN",
        "tan_fn" | "tan" => "TAN",
        "asin_fn" | "asin" => "ASIN",
        "acos_fn" | "acos" => "ACOS",
        "atan_fn" | "atan" => "ATAN",
        "sinh_fn" | "sinh" => "SINH",
        "cosh_fn" | "cosh" => "COSH",
        "tanh_fn" | "tanh" => "TANH",
        "sqrt_fn" | "sqrt" => "SQRT",
        "exp_fn" | "exp" => "EXP",
        "log_fn" | "log" => "LOG",
        "abs_fn" | "abs" => "ABS",
        "norm_fn" | "norm" => "NORM",
        "cons_fn" | "cons" => "CONS",
        "int_fn" => "INT",
        "nint" => "NINT",
        "type_fn" => "TYPE",
        "real_fn" => "REAL",
        "imag_fn" => "IMAG",
        "re_convert" => "RE",
        "string_convert" => "ST",
        "logical_convert" => "LO",
        "complex_convert" => "CM",
        "ve_convert" => "VE",
        "erf" => "ERF",
        "werf" => "WERF",
        "isrt" => "ISRT",
        "isrt3" => "ISRT3",
        "cmplx" => "CMPLX",
        "conj" => "CONJ",
        "trim" => "TRIM",
        "ltrim" => "LTRIM",
        "length" => "LENGTH",
        "varmem" => "VARMEM",
        "varpoi" => "VARPOI",
        "pow" => return None,       // ^ operator, not a keyword
        "neg" => return None,       // unary -, not a keyword
        "not" => return None,       // ! operator
        "add" | "sub" | "mult" | "div" => return None, // arithmetic operators
        "eq" | "neq" | "lt" | "gt" | "lte" | "gte" => return None, // comparison operators
        "concat" | "derive" | "extract" => return None, // collection operators
        "number" | "string" | "boolean" => return None, // literal types
        "cd" => "CD",
        "da" => "DA",
        // Intermediate category directories (not leaf modules)
        "core" | "io" | "math" | "trig" | "exponential" | "rounding"
        | "special" | "vector" | "complex" | "memory" | "query"
        | "conversion" | "sys" | "collection" | "comparison"
        | "arithmetic" | "unary" | "types" | "operators" | "functions" => return None,
        // Default: uppercase the directory name (handles most DA ops, etc.)
        other => return Some(other.to_uppercase()),
    };
    Some(keyword.to_string())
}

// ─── Rule → Keyword Mapping ────────────────────────────────────────────────

/// Build a map from pest rule names to their keyword (for the grammar rules
/// that start with ^"KEYWORD").
fn build_rule_keyword_map(source: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for line in source.lines() {
        let trimmed = line.trim();
        // Match lines like: `sin = { ^"SIN" ~ "(" ~ expr ~ ")" }`
        if let Some((rule_name, rest)) = trimmed.split_once('=') {
            let rule_name = rule_name.trim();
            if let Some(kw) = extract_first_keyword(rest) {
                map.insert(rule_name.to_string(), kw.to_uppercase());
            }
        }
    }

    map
}

/// Extract the first ^"KEYWORD" from a pest rule body.
fn extract_first_keyword(body: &str) -> Option<String> {
    let body = body.trim().trim_start_matches('{').trim();
    // Find ^"..."
    if let Some(pos) = body.find("^\"") {
        let rest = &body[pos + 2..];
        if let Some(end) = rest.find('"') {
            return Some(rest[..end].to_string());
        }
    }
    None
}

// ─── Code Generation ───────────────────────────────────────────────────────

fn generate_keywords_file(
    out_dir: &str,
    keywords: &[String],
    module_docs: &HashMap<String, ModuleDoc>,
) {
    let dest = Path::new(out_dir).join("keywords_generated.rs");
    let mut f = fs::File::create(&dest).unwrap();

    writeln!(f, "// Auto-generated from rosy.pest + module docs by build.rs — do not edit!").unwrap();
    writeln!(f).unwrap();
    writeln!(f, "/// All ROSY keywords extracted from the grammar.").unwrap();
    writeln!(f, "/// Each entry is (keyword, brief description).").unwrap();
    writeln!(f, "pub const ROSY_KEYWORD_LIST: &[(&str, &str)] = &[").unwrap();

    for kw in keywords {
        let desc = module_docs
            .get(kw.as_str())
            .map(|d| {
                if d.description.is_empty() {
                    d.title.clone()
                } else {
                    d.description.clone()
                }
            })
            .unwrap_or_else(|| "ROSY keyword".to_string());
        let desc = desc.replace('\\', "\\\\").replace('"', "\\\"");
        writeln!(f, "    (\"{kw}\", \"{desc}\"),").unwrap();
    }

    writeln!(f, "];").unwrap();
}

fn generate_hover_file(
    out_dir: &str,
    module_docs: &HashMap<String, ModuleDoc>,
    _rule_to_keyword: &HashMap<String, String>,
) {
    let dest = Path::new(out_dir).join("hover_generated.rs");
    let mut f = fs::File::create(&dest).unwrap();

    writeln!(f, "// Auto-generated from module doc comments by build.rs — do not edit!").unwrap();
    writeln!(f).unwrap();
    writeln!(f, "/// Hover documentation for ROSY keywords.").unwrap();
    writeln!(f, "/// Each entry is (keyword, title, description, doc_url, is_statement).").unwrap();
    writeln!(
        f,
        "pub const ROSY_HOVER_DOCS: &[(&str, &str, &str, &str, bool)] = &["
    )
    .unwrap();

    let mut entries: Vec<_> = module_docs.iter().collect();
    entries.sort_by_key(|(k, _)| (*k).clone());

    for (keyword, doc) in &entries {
        let title = doc.title.replace('\\', "\\\\").replace('"', "\\\"");
        let desc = doc.description.replace('\\', "\\\\").replace('"', "\\\"");
        let url = &doc.doc_url;
        let is_stmt = matches!(doc.kind, ModuleKind::Statement);
        writeln!(
            f,
            "    (\"{keyword}\", \"{title}\", \"{desc}\", \"{url}\", {is_stmt}),"
        )
        .unwrap();
    }

    writeln!(f, "];").unwrap();

    // Also generate the type hover docs (these are fixed and small)
    writeln!(f).unwrap();
    writeln!(f, "/// Hover documentation for ROSY type annotations.").unwrap();
    writeln!(
        f,
        "pub const ROSY_TYPE_HOVER: &[(&str, &str, &str)] = &["
    )
    .unwrap();
    let base = BASE_DOC_URL;
    for (name, rust_type, desc) in [
        ("RE", "f64", "Real number"),
        ("ST", "String", "String"),
        ("LO", "bool", "Logical / boolean"),
        ("CM", "Complex64", "Complex number"),
        ("VE", "Vec<f64>", "Vector"),
        ("DA", "Taylor series", "Differential Algebra"),
        ("CD", "Complex Taylor series", "Complex Differential Algebra"),
    ] {
        writeln!(
            f,
            "    (\"{name}\", \"**{name}** \\u{{2014}} {desc} (`{rust_type}`)\\n\\n[Documentation]({base}/rosy_lib/enum.RosyBaseType.html#variant.{name})\", \"{desc}\"),"
        )
        .unwrap();
    }
    writeln!(f, "];").unwrap();
}
