pub mod add;
pub mod extract;
pub mod concat;

pub use add::RosyAdd;
pub use concat::RosyConcat;
pub use extract::RosyExtract;

use std::collections::HashMap;
use crate::rosy_lib::{RosyType, RosyBaseType};

/// Defines a type compatibility rule for an operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeRule {
    /// Left-hand side type
    pub lhs: &'static str,
    /// Right-hand side type
    pub rhs: &'static str,
    /// Result type
    pub result: &'static str,
    /// Optional comment for documentation
    pub comment: &'static str,
}

impl TypeRule {
    /// Create a new type rule without a comment.
    pub const fn new(lhs: &'static str, rhs: &'static str, result: &'static str) -> Self {
        Self { lhs, rhs, result, comment: "" }
    }
    
    /// Create a new type rule with a comment.
    pub const fn with_comment(
        lhs: &'static str,
        rhs: &'static str,
        result: &'static str,
        comment: &'static str,
    ) -> Self {
        Self { lhs, rhs, result, comment }
    }
}

/// Convert a type string to RosyType.
/// 
/// This is used by operator registries to convert type rule strings
/// into actual RosyType instances for runtime lookups.
pub fn type_from_str(s: &str) -> RosyType {
    match s {
        "RE" => RosyType::new(RosyBaseType::RE, 0),
        "ST" => RosyType::new(RosyBaseType::ST, 0),
        "LO" => RosyType::new(RosyBaseType::LO, 0),
        "CM" => RosyType::new(RosyBaseType::CM, 0),
        "VE" => RosyType::new(RosyBaseType::VE, 0),
        "DA" => RosyType::new(RosyBaseType::DA, 0),
        "CD" => RosyType::new(RosyBaseType::CD, 0),
        _ => panic!("Unknown type: {}", s),
    }
}

/// Build a type compatibility registry from a slice of TypeRules.
/// 
/// This is a helper function used by operators to convert their const
/// TypeRule arrays into runtime HashMap lookups.
pub fn build_type_registry(rules: &[TypeRule]) -> HashMap<(RosyType, RosyType), RosyType> {
    let mut m = HashMap::new();
    for rule in rules {
        m.insert(
            (type_from_str(rule.lhs), type_from_str(rule.rhs)), 
            type_from_str(rule.result)
        );
    }
    m
}

/// Macro to define operator registries more concisely.
/// 
/// # Example
/// ```ignore
/// operator_registry! {
///     ADD => [
///         RE + RE => RE,
///         RE + CM => CM,
///         CM + DA => CD ("Complex DA result"),
///     ]
/// }
/// ```
#[macro_export]
macro_rules! operator_registry {
    // Entry with comment
    ($lhs:ident + $rhs:ident => $result:ident ($comment:literal)) => {
        $crate::operators::TypeRule::with_comment(
            stringify!($lhs),
            stringify!($rhs),
            stringify!($result),
            $comment
        )
    };
    
    // Entry without comment
    ($lhs:ident + $rhs:ident => $result:ident) => {
        $crate::operators::TypeRule::new(
            stringify!($lhs),
            stringify!($rhs),
            stringify!($result)
        )
    };
}

#[cfg(test)]
pub mod test_utils {
    use std::process::Command;
    use std::path::PathBuf;

    /// Test that ROSY and COSY outputs match for a given operator.
    /// 
    /// This is the shared test framework used by all operator modules.
    /// 
    /// # Arguments
    /// * `operator_name` - Name of the operator (e.g., "add", "concat")
    /// 
    /// # Example
    /// ```ignore
    /// #[test]
    /// fn test_rosy_cosy_output_match() {
    ///     test_operator_output_match("add");
    /// }
    /// ```
    pub fn test_operator_output_match(operator_name: &str) {
        // Get workspace root
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("Failed to get workspace root")
            .to_path_buf();
        
        let rosy_script = workspace_root.join(format!("rosy/assets/operators/{}/{}.rosy", operator_name, operator_name));
        let cosy_script = workspace_root.join(format!("rosy/assets/operators/{}/{}.fox", operator_name, operator_name));
        let rosy_transpiler = workspace_root.join("target/release/rosy_transpiler");
        let cosy_exe = workspace_root.join("cosy");

        // Check if executables exist
        if !rosy_transpiler.exists() {
            panic!("rosy_transpiler not found at {:?}. Run: cargo build --release -p rosy_transpiler", rosy_transpiler);
        }
        if !cosy_exe.exists() {
            panic!("cosy executable not found at {:?}", cosy_exe);
        }

        // Run ROSY transpiler
        let transpile_output = Command::new(&rosy_transpiler)
            .arg(&rosy_script)
            .current_dir(&workspace_root)
            .output()
            .expect("Failed to run rosy_transpiler");

        assert!(
            transpile_output.status.success(),
            "ROSY transpiler failed:\n{}",
            String::from_utf8_lossy(&transpile_output.stderr)
        );

        // Run the transpiled ROSY code
        let rosy_output = Command::new("cargo")
            .args(&["run", "--release"])
            .current_dir(workspace_root.join(".rosy_output"))
            .output()
            .expect("Failed to run transpiled ROSY code");

        assert!(
            rosy_output.status.success(),
            "ROSY execution failed:\n{}",
            String::from_utf8_lossy(&rosy_output.stderr)
        );

        let rosy_stdout = String::from_utf8_lossy(&rosy_output.stdout);
        let rosy_lines: Vec<&str> = rosy_stdout.lines().collect();

        // Run COSY script
        let cosy_output = Command::new(&cosy_exe)
            .arg(&cosy_script)
            .current_dir(&workspace_root)
            .output()
            .expect("Failed to run COSY");

        assert!(
            cosy_output.status.success(),
            "COSY execution failed:\n{}",
            String::from_utf8_lossy(&cosy_output.stderr)
        );

        let cosy_stdout = String::from_utf8_lossy(&cosy_output.stdout);
        
        // Extract just the test output lines from COSY
        // Skip the banner and compilation messages by splitting on "BEGINNING EXECUTION"
        let cosy_output_after_exec = cosy_stdout
            .split("--- BEGINNING EXECUTION")
            .nth(1)
            .unwrap_or("");
        
        let cosy_lines: Vec<&str> = cosy_output_after_exec
            .lines()
            .skip(1)  // Skip the first blank line after "--- BEGINNING EXECUTION"
            .collect();

        // Compare outputs
        println!("\n=== ROSY Output ({} lines) ===", rosy_lines.len());
        for (i, line) in rosy_lines.iter().enumerate() {
            println!("{:3}: {}", i, line);
        }

        println!("\n=== COSY Output ({} lines) ===", cosy_lines.len());
        for (i, line) in cosy_lines.iter().enumerate() {
            println!("{:3}: {}", i, line);
        }

        println!("\n=== Comparison ===");
        let max_lines = rosy_lines.len().max(cosy_lines.len());
        let mut differences = Vec::new();

        for i in 0..max_lines {
            let rosy_line = rosy_lines.get(i).map(|s| s.trim()).unwrap_or("<missing>");
            let cosy_line = cosy_lines.get(i).map(|s| s.trim()).unwrap_or("<missing>");

            if rosy_line != cosy_line {
                differences.push(format!(
                    "Line {}: \n  ROSY: {}\n  COSY: {}",
                    i, rosy_line, cosy_line
                ));
            }
        }

        if !differences.is_empty() {
            println!("\nFound {} differences:", differences.len());
            for diff in &differences {
                println!("{}", diff);
            }
            panic!("ROSY and COSY outputs do not match for operator '{}'!", operator_name);
        }

        println!("\n✅ All {} lines match for operator '{}'!", rosy_lines.len(), operator_name);
    }
}