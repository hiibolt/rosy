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

        // Run ROSY transpiler
        let transpile_output = Command::new("cargo")
            .arg("run")
            .arg("-p")
            .arg("rosy")
            .arg("--")
            .arg(&rosy_script)
            .current_dir(&workspace_root)
            .output()
            .expect("Failed to run rosy");

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
            panic!("ROSY and COSY outputs do not match for intrinsic '{}'!", intrinsic_name);
        }

        println!("\n✅ All {} lines match for intrinsic '{}'!", rosy_lines.len(), intrinsic_name);
    }
}
