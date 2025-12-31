pub mod cm;
pub mod st;
pub mod lo;
pub mod from_st;
pub mod length;
pub mod sin;

pub use cm::RosyCM;
pub use st::RosyST;
pub use lo::RosyLO;
pub use from_st::RosyFromST;
pub use length::RosyLENGTH;
pub use sin::RosySIN;

/// Represents a parsed intrinsic type rule from the source code.
#[derive(Debug, Clone)]
pub struct IntrinsicTypeRule {
    pub input: &'static str,
    pub result: &'static str,
    pub test_val: &'static str,
}
impl IntrinsicTypeRule {
    /// Create a new intrinsic type rule.
    pub const fn new(
        input: &'static str,
        result: &'static str,
        test_val: &'static str
    ) -> Self {
        Self { input, result, test_val }
    }
}

#[cfg(test)]
pub mod test_utils {
    use std::process::Command;
    use std::path::PathBuf;

    use crate::rosy_lib::intrinsics::extract_numbers;

    /// Test that ROSY and COSY outputs match for a given intrinsic function.
    /// 
    /// This is the shared test framework used by all intrinsic modules.
    /// 
    /// # Arguments
    /// * `intrinsic_name` - Name of the intrinsic (e.g., "length")
    /// 
    /// # Example
    /// ```ignore
    /// #[test]
    /// fn test_rosy_cosy_output_match() {
    ///     test_intrinsic_output_match("length");
    /// }
    /// ```
    pub fn test_intrinsic_output_match(intrinsic_name: &str) {
        // Get workspace root
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("Failed to get workspace root")
            .to_path_buf();
        
        let rosy_script = workspace_root.join(format!("rosy/assets/intrinsics/{}/{}.rosy", intrinsic_name, intrinsic_name));
        let cosy_script = workspace_root.join(format!("rosy/assets/intrinsics/{}/{}.fox", intrinsic_name, intrinsic_name));
        let cosy_exe = workspace_root.join("cosy");

        // Check if executables exist
        if !cosy_exe.exists() {
            panic!("cosy executable not found at {:?}", cosy_exe);
        }

        // Use isolated build directory for this test to allow parallel execution
        let test_build_dir = workspace_root.join(format!(".rosy_test_cache/{}", intrinsic_name));

        // Run ROSY transpiler with isolated build directory
        let transpile_output = Command::new("cargo")
            .arg("run")
            .arg("--release")
            .arg("-p")
            .arg("rosy")
            .arg("--")
            .arg("run")
            .arg(&rosy_script)
            .arg("-d")
            .arg(&test_build_dir)
            .current_dir(&workspace_root)
            .output()
            .expect("Failed to run rosy");

        assert!(
            transpile_output.status.success(),
            "ROSY transpiler failed:\n{}",
            String::from_utf8_lossy(&transpile_output.stderr)
        );

        // The rosy transpile_output contains the execution output directly
        let rosy_stdout = String::from_utf8_lossy(&transpile_output.stdout);
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

            // Check if lines are different
            if rosy_line != cosy_line {
                // Try numeric comparison for lines with floating point values
                if let (Some(rosy_nums), Some(cosy_nums)) = (extract_numbers(rosy_line), extract_numbers(cosy_line)) {
                    // If we have the same count of numbers, compare with tolerance
                    if rosy_nums.len() == cosy_nums.len() {
                        let all_close = rosy_nums.iter().zip(cosy_nums.iter()).all(|(r, c)| {
                            let diff = (r - c).abs();
                            let max_val = r.abs().max(c.abs());
                            // Relative tolerance of 1e-6 or absolute tolerance of 1e-9
                            diff < 1e-9 || diff / max_val < 1e-6
                        });
                        
                        if all_close {
                            // Numbers are within tolerance, skip this difference
                            continue;
                        }
                    }
                }
                
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
            panic!("ROSY and COSY outputs do not match for intrinsic '{}'!", intrinsic_name);
        }

        println!("\n✅ All {} lines match for intrinsic '{}'!", rosy_lines.len(), intrinsic_name);
    }
}

/// Extract floating point numbers from a line for numeric comparison.
/// Returns None if the line doesn't contain parseable numbers.
fn extract_numbers(line: &str) -> Option<Vec<f64>> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let mut numbers = Vec::new();
    
    for part in parts {
        // Try to parse as f64, handling scientific notation
        if let Ok(num) = part.parse::<f64>() {
            numbers.push(num);
        }
    }
    
    if numbers.is_empty() {
        None
    } else {
        Some(numbers)
    }
}
