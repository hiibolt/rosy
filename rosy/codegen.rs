//! Build-time code generation from `rosy_test_raw` doc-comment annotations.
//!
//! Walks source directories for `rosy_test_raw` blocks and generates:
//! - ROSY test scripts (`*.rosy`)
//! - COSY test scripts (`*.fox`)
//! - Per-construct documentation (`*_doc.md`) with code + expected output
//! - Combined category docs (`*_docs.md`) included in rustdoc
//! - Test runner (`annotated_tests.rs`) with per-construct `#[test]` functions

use std::fs;
use std::path::Path;

/// Parsed `rosy_test_raw` annotation from module doc comments.
#[derive(Debug, Clone)]
pub struct RosyTestAnnotation {
    pub rosy_code: String,
    pub fox_code: String,
}

/// Parse a `rosy_test_raw` annotation from a source file's doc comments.
///
/// Looks for blocks of the form:
/// ```text
/// //! ```rosy_test_raw
/// //! --- rosy ---
/// //! <rosy code>
/// //! --- fox ---
/// //! <fox code>
/// //! ```
/// ```
pub fn parse_rosy_test_annotation(source_path: &Path) -> Option<RosyTestAnnotation> {
    let content = fs::read_to_string(source_path).ok()?;

    let mut in_block = false;
    let mut in_rosy = false;
    let mut in_fox = false;
    let mut rosy_lines: Vec<String> = Vec::new();
    let mut fox_lines: Vec<String> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Accept both //! (module-level) and /// (item-level) doc comments
        let doc_content = if let Some(rest) = trimmed.strip_prefix("//!") {
            rest.strip_prefix(' ').unwrap_or(rest)
        } else if let Some(rest) = trimmed.strip_prefix("///") {
            rest.strip_prefix(' ').unwrap_or(rest)
        } else {
            continue;
        };

        if !in_block && doc_content.trim() == "```rosy_test_raw" {
            in_block = true;
            in_rosy = false;
            in_fox = false;
            continue;
        }

        if in_block && doc_content.trim() == "```" {
            break; // End of annotation block
        }

        if in_block {
            if doc_content.trim() == "--- rosy ---" {
                in_rosy = true;
                in_fox = false;
                continue;
            }
            if doc_content.trim() == "--- fox ---" {
                in_fox = true;
                in_rosy = false;
                continue;
            }

            if in_rosy {
                rosy_lines.push(doc_content.to_string());
            } else if in_fox {
                fox_lines.push(doc_content.to_string());
            }
        }
    }

    if rosy_lines.is_empty() || fox_lines.is_empty() {
        return None;
    }

    Some(RosyTestAnnotation {
        rosy_code: rosy_lines.join("\n"),
        fox_code: fox_lines.join("\n"),
    })
}

/// Walk a directory tree, find .rs files with `rosy_test_raw` annotations,
/// and generate test files for each discovered annotation.
///
/// Returns list of `(name, annotation)` pairs for test manifest generation.
pub fn discover_and_codegen_annotated(
    base_dir: &Path,
    category: &str,
) -> Vec<(String, RosyTestAnnotation)> {
    let mut results = Vec::new();
    walk_for_annotations(base_dir, &mut results);
    results.sort_by(|a, b| a.0.cmp(&b.0));

    for (name, annotation) in &results {
        codegen_annotated(category, name, annotation);
    }

    results
}

fn walk_for_annotations(
    dir: &Path,
    results: &mut Vec<(String, RosyTestAnnotation)>,
) {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .collect();
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            walk_for_annotations(&path, results);
        } else if path.extension().map_or(false, |e| e == "rs") {
            let stem = path.file_stem().unwrap().to_string_lossy().to_string();
            if stem == "mod" { continue; }

            if let Some(annotation) = parse_rosy_test_annotation(&path) {
                println!("cargo:rerun-if-changed={}", path.display());
                results.push((stem, annotation));
            }
        }
    }
}

/// Generate test files (`.rosy`, `.fox`, `_doc.md`) from an annotation.
///
/// If a `.expected` file already exists (version-tracked), includes it in the
/// generated documentation.
pub fn codegen_annotated(category: &str, name: &str, annotation: &RosyTestAnnotation) {
    let dir = Path::new("assets").join(category).join(name);
    fs::create_dir_all(&dir).expect("Failed to create assets directory");

    // Write .rosy file
    fs::write(dir.join(format!("{}.rosy", name)), &annotation.rosy_code)
        .expect("Failed to write ROSY script");

    // Write .fox file
    fs::write(dir.join(format!("{}.fox", name)), &annotation.fox_code)
        .expect("Failed to write COSY script");

    // Generate documentation with code and any existing expected output
    let mut md = format!("# {}\n\n", name.to_uppercase().replace('_', " "));
    md.push_str("## ROSY Test\n\n```rosy\n");
    md.push_str(&annotation.rosy_code);
    md.push_str("\n```\n\n");

    let expected_path = dir.join(format!("{}.expected", name));
    if expected_path.exists() {
        if let Ok(expected) = fs::read_to_string(&expected_path) {
            md.push_str("## Expected Output\n\n```\n");
            md.push_str(&expected);
            if !expected.ends_with('\n') { md.push('\n'); }
            md.push_str("```\n\n");
        }
    }

    md.push_str("## COSY Equivalent\n\n```cosy\n");
    md.push_str(&annotation.fox_code);
    md.push_str("\n```\n");

    fs::write(dir.join(format!("{}_doc.md", name)), md)
        .expect("Failed to write doc file");

    println!("cargo:warning=Generated annotated test files for '{}/{}'", category, name);
}

/// Generate the auto-test runner file that will be `include!`-d in
/// the annotated test module.
///
/// Each discovered annotation gets its own `#[test]` function that
/// transpiles, compiles, and runs the `.rosy` script, then writes or
/// compares against a `.expected` file.
pub fn generate_annotated_test_runner(
    all_tests: &[(&str, &[(String, RosyTestAnnotation)])],
) {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("annotated_tests.rs");

    use std::io::Write;
    let mut f = fs::File::create(&dest).unwrap();

    writeln!(f, "// Auto-generated by build.rs - do not edit manually!").unwrap();
    writeln!(f).unwrap();

    let mut total = 0;
    for &(category, ref tests) in all_tests {
        for (name, _) in tests.iter() {
            writeln!(f, "#[test]").unwrap();
            writeln!(f, "fn test_{}_{}() {{", category, name).unwrap();
            writeln!(f, "    test_annotated_rosy_output(\"{}\", \"{}\");", category, name).unwrap();
            writeln!(f, "}}").unwrap();
            writeln!(f).unwrap();
            total += 1;
        }
    }

    println!("cargo:warning=Generated {} annotated test functions", total);
}

/// Generate a combined documentation file for a category.
///
/// Concatenates all per-construct `_doc.md` files into one `{category}_docs.md`
/// that can be `include_str!`-d in module-level rustdoc.
pub fn generate_combined_docs(category: &str, tests: &[(String, RosyTestAnnotation)]) {
    let assets = Path::new("assets");

    let mut combined = String::new();
    for (name, _) in tests {
        let doc_path = assets.join(category).join(name).join(format!("{}_doc.md", name));
        if let Ok(content) = fs::read_to_string(&doc_path) {
            combined.push_str(&content);
            combined.push_str("\n---\n\n");
        }
    }

    fs::write(assets.join(format!("{}_docs.md", category)), combined)
        .expect("Failed to write combined docs");
}
