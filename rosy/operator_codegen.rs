//! Build-time code generation for operator tests.
//!
//! This module parses the operator registries defined in `src/rosy_lib/operators/*/mod.rs`
//! and generates:
//! - Documentation tables (*.md)
//! - ROSY test scripts (*.rosy)
//! - COSY test scripts (*.fox)
//! - Expected output files (*.expected)

use std::fs;
use std::path::Path;

/// Represents a parsed type rule from the source code.
#[derive(Debug, Clone)]
pub struct TypeRule {
    pub lhs: String,
    pub rhs: String,
    pub result: String,
    pub comment: String,
}

/// Parse ADD_REGISTRY from add/mod.rs source code.
/// 
/// This uses simple regex parsing to extract TypeRule::new() calls.
pub fn parse_registry_from_source(source_path: &Path) -> Vec<TypeRule> {
    let content = fs::read_to_string(source_path)
        .expect("Failed to read operator source file");
    
    let mut rules = Vec::new();
    
    // Simple parser: look for TypeRule::new("X", "Y", "Z") patterns
    for line in content.lines() {
        if line.trim_start().starts_with("TypeRule::new(") {
            if let Some(rule) = parse_type_rule_new(line) {
                rules.push(rule);
            }
        } else if line.trim_start().starts_with("TypeRule::with_comment(") {
            if let Some(rule) = parse_type_rule_with_comment(line) {
                rules.push(rule);
            }
        }
    }
    
    rules
}

fn parse_type_rule_new(line: &str) -> Option<TypeRule> {
    // Parse: TypeRule::new("RE", "CM", "VE"),
    let start = line.find("TypeRule::new(")?;
    let content = &line[start + "TypeRule::new(".len()..];
    let end = content.find(")")?;
    let args = &content[..end];
    
    let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
    if parts.len() != 3 {
        return None;
    }
    
    Some(TypeRule {
        lhs: parts[0].trim_matches('"').to_string(),
        rhs: parts[1].trim_matches('"').to_string(),
        result: parts[2].trim_matches('"').to_string(),
        comment: String::new(),
    })
}

fn parse_type_rule_with_comment(line: &str) -> Option<TypeRule> {
    // Parse: TypeRule::with_comment("RE", "VE", "VE", "Add Real componentwise"),
    let start = line.find("TypeRule::with_comment(")?;
    let content = &line[start + "TypeRule::with_comment(".len()..];
    let end = content.find(")")?;
    let args = &content[..end];
    
    let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
    if parts.len() != 4 {
        return None;
    }
    
    Some(TypeRule {
        lhs: parts[0].trim_matches('"').to_string(),
        rhs: parts[1].trim_matches('"').to_string(),
        result: parts[2].trim_matches('"').to_string(),
        comment: parts[3].trim_matches('"').to_string(),
    })
}

/// Generate markdown documentation table from registry.
pub fn generate_doc_table(rules: &[TypeRule]) -> String {
    let mut table = String::from(
        "| Left | Right | Result | Comment |\n\
         |---|---|---|---|\n"
    );
    
    for rule in rules {
        table.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            rule.lhs,
            rule.rhs,
            rule.result,
            rule.comment
        ));
    }
    
    table
}

/// Get the operator symbol for a given operator name.
fn get_operator_symbol(operator_name: &str) -> &'static str {
    match operator_name {
        "add" => "+",
        "concat" => "&",
        "extract" => "j",
        _ => panic!("Unknown operator name: {}", operator_name),
    }
}

/// Generate ROSY test script.
/// 
/// For each type combination, creates variables and performs the operation.
/// Uses language-appropriate test values for each type.
pub fn generate_rosy_script(operator_name: &str, rules: &[TypeRule]) -> String {
    let op_symbol = get_operator_symbol(operator_name);
    let mut script = String::from("BEGIN;\n");
    
    for (idx, rule) in rules.iter().enumerate() {
        script.push_str(&format!("    VARIABLE ({}) LHS_{};\n", rule.lhs, idx));
        script.push_str(&format!("    VARIABLE ({}) RHS_{};\n", rule.rhs, idx));
        script.push_str(&format!("    VARIABLE ({}) RESULT_{};\n\n", rule.result, idx));
        
        // Initialize with type-appropriate test values
        script.push_str(&format!("    LHS_{} := {};\n", idx, get_rosy_test_value(&rule.lhs)));
        script.push_str(&format!("    RHS_{} := {};\n", idx, get_rosy_test_value(&rule.rhs)));
        
        // Perform operation
        script.push_str(&format!("    RESULT_{} := LHS_{} {} RHS_{};\n", idx, idx, op_symbol, idx));
        
        // Write the result (just output the result value for now)
        // Can't write literal strings in ROSY without ST() conversion
        script.push_str(&format!("    WRITE 6 ST(RESULT_{});\n\n", idx));
    }
    
    script.push_str("END;\n");
    script
}

/// Generate COSY INFINITY test script.
pub fn generate_cosy_script(operator_name: &str, rules: &[TypeRule]) -> String {
    let op_symbol = get_operator_symbol(operator_name);
    let mut script = String::from("BEGIN;\n\nPROCEDURE RUN;\n");
    
    // FOX/COSY requires ALL variable declarations at procedure start
    script.push_str("    { All variable declarations must come first in FOX/COSY }\n");
    script.push_str("    VARIABLE NM 1;\n");
    
    // First pass: declare all variables
    for (idx, rule) in rules.iter().enumerate() {
        script.push_str(&format!("    VARIABLE LHS_{} {};\n", idx, get_cosy_var_size(&rule.lhs)));
        script.push_str(&format!("    VARIABLE RHS_{} {};\n", idx, get_cosy_var_size(&rule.rhs)));
        script.push_str(&format!("    VARIABLE RESULT_{} {};\n", idx, get_cosy_var_size(&rule.result)));
    }
    
    // Initialize DA system (needed for DA/CD types)
    script.push_str("\n    { Initialize DA system for tests that use DA/CD types }\n");
    script.push_str("    { DAINI: order 2, number_of_variables 2, mode 0 (see COSY manual) }\n");
    script.push_str("    DAINI 2 2 0 NM;\n\n");
    
    // Second pass: assignments and operations
    for (idx, rule) in rules.iter().enumerate() {
        script.push_str(&format!("    {{ Test {}: {} {} {} => {} }}\n", idx, rule.lhs, op_symbol, rule.rhs, rule.result));
        script.push_str(&format!("    LHS_{} := {};\n", idx, get_cosy_test_value(&rule.lhs)));
        script.push_str(&format!("    RHS_{} := {};\n", idx, get_cosy_test_value(&rule.rhs)));
        script.push_str(&format!("    RESULT_{} := LHS_{} {} RHS_{};\n", idx, idx, op_symbol, idx));
        script.push_str(&format!("    WRITE 6 RESULT_{};\n\n", idx));
    }
    
    script.push_str("ENDPROCEDURE;\n\nRUN;\nEND;\n");
    script
}

/// Get appropriate test value for a ROSY type.
/// 
/// ROSY uses & (concat) to build composite types:
/// - CM: CM(real & imag) - Complex from real & imaginary
/// - VE: val1 & val2 & val3 - Vector concatenation
/// - DA: DA(var_index) - creates DA for differential variable
/// - CD: DA(1) & DA(2) - Complex DA from two DAs concatenated
fn get_rosy_test_value(type_name: &str) -> &'static str {
    match type_name {
        "RE" => "2.5",
        "CM" => "CM(1.0 & 2.0)",  // Complex: real & imaginary
        "VE" => "1.0 & 2.0 & 3.0",  // Vector: concatenate with &
        "DA" => "DA(1)",  // DA(var_index) creates differential variable
        "CD" => "DA(1) & DA(2)",  // Complex DA: DA & DA concatenation
        "LO" => "TRUE",
        "ST" => "\"test\"",
        _ => "0.0",
    }
}

/// Get appropriate test value for COSY type.
fn get_cosy_test_value(type_name: &str) -> &'static str {
    match type_name {
        "RE" => "2.5",
        "CM" => "CM(1.0&2.0)",  // COSY complex: CM(real&imaginary)
        "VE" => "1.0",  // COSY vectors are just scalars
        "DA" => "DA(1)",  // Use DA(var_index) to create DA variable
        "CD" => "DA(1)+CM(0&1)*DA(2)",  // Complex DA: real part + i*imaginary part
        "LO" => "1",
        "ST" => "'test'",  // COSY uses single quotes for strings
        _ => "0.0",
    }
}

/// Get COSY variable size (for VARIABLE X <size>).
fn get_cosy_var_size(type_name: &str) -> &'static str {
    match type_name {
        "CM" => "2",   // Complex needs 2 slots (real + imaginary)
        "VE" => "3",   // Vector of size 3
        "DA" => "100", // DA needs space for coefficients
        "CD" => "100", // Complex DA needs space for DA coefficients
        _ => "1",      // Everything else is scalar
    }
}

/// Run all code generation for an operator.
pub fn codegen_operator(operator_name: &str) {
    let src_path = Path::new("src/rosy_lib/operators")
        .join(format!("{}.rs", operator_name));
    
    let operator_dir = Path::new("assets/operators").join(operator_name);
    
    // Create the assets directory if it doesn't exist
    fs::create_dir_all(&operator_dir)
        .expect("Failed to create assets directory");
    
    println!("cargo:rerun-if-changed={}", src_path.display());
    
    // Parse registry from source
    let rules = parse_registry_from_source(&src_path);
    
    if rules.is_empty() {
        println!("cargo:warning=No registry found in {}", src_path.display());
        return;
    }
    
    println!("cargo:warning=Generating {} tests for operator '{}'", rules.len(), operator_name);
    
    // Generate documentation table
    let doc_table = generate_doc_table(&rules);
    fs::write(operator_dir.join(format!("{}_table.md", operator_name)), doc_table)
        .expect("Failed to write doc table");
    
    // Generate ROSY script
    let rosy_script = generate_rosy_script(operator_name, &rules);
    fs::write(operator_dir.join(format!("{}.rosy", operator_name)), rosy_script)
        .expect("Failed to write ROSY script");
    
    // Generate COSY script
    let cosy_script = generate_cosy_script(operator_name, &rules);
    fs::write(operator_dir.join(format!("{}.fox", operator_name)), cosy_script)
        .expect("Failed to write COSY script");
    
    println!("cargo:warning=Generated test files for operator '{}'", operator_name);
}
