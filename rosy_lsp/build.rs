//! Extracts the keyword list from rosy.pest at compile time so that
//! completions stay in sync with the grammar automatically.

use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let pest_path = Path::new("../rosy/assets/rosy.pest");
    println!("cargo:rerun-if-changed={}", pest_path.display());

    let pest_source = fs::read_to_string(pest_path).expect("Failed to read rosy.pest");

    // Extract keywords from the `keyword_raw` rule
    let keywords = extract_keywords(&pest_source);

    // Extract doc comments from the pest grammar (lines starting with `///`)
    let docs = extract_pest_docs(&pest_source);

    // Generate Rust source
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("keywords_generated.rs");
    let mut f = fs::File::create(&dest).unwrap();

    writeln!(f, "// Auto-generated from rosy.pest by build.rs — do not edit!").unwrap();
    writeln!(f).unwrap();
    writeln!(f, "/// All ROSY keywords extracted from the grammar.").unwrap();
    writeln!(f, "pub const ROSY_KEYWORD_LIST: &[(&str, &str)] = &[").unwrap();

    for kw in &keywords {
        let doc = docs
            .get(kw.as_str())
            .map(|s| s.as_str())
            .unwrap_or("ROSY keyword");
        // Escape any quotes in the doc string
        let doc_escaped = doc.replace('\\', "\\\\").replace('"', "\\\"");
        writeln!(f, "    (\"{kw}\", \"{doc_escaped}\"),").unwrap();
    }

    writeln!(f, "];").unwrap();
}

/// Parse the `keyword_raw` rule from the pest grammar and extract all keywords.
fn extract_keywords(source: &str) -> Vec<String> {
    let mut keywords = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("keyword_raw") {
            // Extract everything between { and }
            if let Some(start) = trimmed.find('{') {
                let body = &trimmed[start + 1..];
                let body = body.trim_end_matches('}').trim();

                // Split on | and extract quoted strings
                for part in body.split('|') {
                    let part = part.trim();
                    // Match ^"KEYWORD" or "keyword" patterns
                    if let Some(s) = extract_quoted(part) {
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

/// Extract the string inside ^"..." or "..." from a pest alternative.
fn extract_quoted(s: &str) -> Option<String> {
    let s = s.trim().trim_start_matches('^');
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        Some(s[1..s.len() - 1].to_string())
    } else {
        None
    }
}

/// Extract `/// Comment` lines that precede rules in the pest grammar.
/// Returns a map from keyword name (uppercase) to the doc comment text.
fn extract_pest_docs(source: &str) -> std::collections::HashMap<String, String> {
    let mut docs = std::collections::HashMap::new();
    let mut pending_doc = String::new();

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("///") {
            let comment = trimmed.trim_start_matches('/').trim();
            if !pending_doc.is_empty() {
                pending_doc.push_str("; ");
            }
            pending_doc.push_str(comment);
        } else if !trimmed.is_empty() {
            if !pending_doc.is_empty() {
                // Try to extract the rule name and map it to the keyword
                if let Some(rule_name) = trimmed.split_whitespace().next() {
                    let upper = rule_name.to_uppercase();
                    docs.insert(upper, pending_doc.clone());
                }
                pending_doc.clear();
            }
        }
    }

    docs
}
