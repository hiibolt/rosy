//! # ROSY Runtime Library
//!
//! The embedded runtime library that ships with every generated Rust project.
//! Contains operator implementations, intrinsic functions, type definitions,
//! MPI support, Taylor series (DA/CD), and the optimizer.
//!
//! ## Type Aliases
//!
//! | ROSY Type | Rust Type | Description |
//! |-----------|-----------|-------------|
//! | `RE` | `f64` | Real number |
//! | `ST` | `String` | String |
//! | `LO` | `bool` | Logical (boolean) |
//! | `CM` | `Complex64` | Complex number |
//! | `VE` | `Vec<f64>` | Vector of reals |
//! | `DA` | [`taylor::DA`] | Differential Algebra (Taylor series) |
//! | `CD` | [`taylor::CD`] | Complex Differential Algebra |
//!
//! ## Sub-modules
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`operators`] | Binary operator dispatch (add, sub, mult, div, etc.) |
//! | [`intrinsics`] | Built-in math functions (sin, sqr, exp, etc.) |
//! | [`core`] | Core I/O, file management, concatenation |
//! | [`taylor`] | DA/CD Taylor series implementation |
//! | [`mpi`] | MPI parallel context |
//! | [`optimizer`] | FIT loop optimization algorithms |

pub mod operators;
pub mod intrinsics;
pub mod core;
#[cfg(feature = "mpi")]
pub mod mpi;
pub mod taylor;
pub mod optimizer;

pub use operators::*;
pub use intrinsics::*;
pub use core::*;
#[cfg(feature = "mpi")]
pub use mpi::*;

pub use taylor::{DA, CD};
pub type RE = f64;
pub type ST = String;
pub type LO = bool;
pub type CM = num_complex::Complex64;
pub type VE = Vec<f64>;

#[cfg(test)]
mod annotated_tests {
    use std::process::Command;
    use std::path::PathBuf;
    use std::fs;

    fn test_annotated_rosy_output(category: &str, name: &str) {
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("Failed to get workspace root")
            .to_path_buf();

        let rosy_script = workspace_root.join(format!(
            "rosy/assets/{}/{}/{}.rosy", category, name, name
        ));
        let expected_path = workspace_root.join(format!(
            "rosy/assets/{}/{}/{}.expected", category, name, name
        ));

        assert!(rosy_script.exists(), "ROSY script not found: {:?}", rosy_script);

        let test_build_dir = workspace_root.join(format!(
            ".rosy_test_cache/{}_{}", category, name
        ));
        fs::create_dir_all(&test_build_dir).ok();

        // Transpile and run using test_build_dir as cwd so temp files are isolated
        let output = Command::new("cargo")
            .arg("run").arg("--release")
            .arg("--manifest-path").arg(workspace_root.join("Cargo.toml"))
            .arg("-p").arg("rosy")
            .arg("--").arg("run").arg(&rosy_script).arg("-d").arg(&test_build_dir)
            .current_dir(&test_build_dir)
            .output()
            .expect("Failed to run rosy");

        assert!(
            output.status.success(),
            "ROSY transpiler failed for {}/{}:\nstdout: {}\nstderr: {}",
            category, name,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();

        // Compare against expected output if it exists, otherwise create it
        if expected_path.exists() {
            let expected = fs::read_to_string(&expected_path).unwrap();
            assert_eq!(
                stdout.trim(), expected.trim(),
                "Output mismatch for {}/{}.\nExpected:\n{}\nGot:\n{}",
                category, name, expected.trim(), stdout.trim()
            );
        } else {
            fs::write(&expected_path, &stdout)
                .unwrap_or_else(|e| panic!(
                    "Failed to write expected output for {}/{}: {}", category, name, e
                ));
            println!("Created expected output file for {}/{}: {:?}", category, name, expected_path);
        }
    }

    include!(concat!(env!("OUT_DIR"), "/annotated_tests.rs"));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RosyType {
    pub base_type: RosyBaseType,
    pub dimensions: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RosyBaseType {
    RE,
    ST,
    LO,
    CM,
    VE,
    DA,
    CD,
}
impl std::fmt::Display for RosyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.dimensions == 0 {
            write!(f, "({:?})", self.base_type)
        } else {
            let dims = "*".repeat(self.dimensions);
            write!(f, "({:?} {dims})", self.base_type)
        }
    }
}
impl RosyType {
    pub fn new ( base_type: RosyBaseType, dimensions: usize ) -> Self {
        RosyType {
            base_type,
            dimensions
        }
    }

    #[allow(non_snake_case)]
    pub fn RE ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::RE,
            dimensions: 0
        }
    }
    #[allow(non_snake_case)]
    pub fn ST ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::ST,
            dimensions: 0
        }
    }
    #[allow(non_snake_case)]
    pub fn LO ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::LO,
            dimensions: 0
        }
    }
    #[allow(non_snake_case)]
    pub fn CM ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::CM,
            dimensions: 0
        }
    }
    #[allow(non_snake_case)]
    pub fn VE ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::VE,
            dimensions: 0
        }
    }
    #[allow(non_snake_case)]
    pub fn DA ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::DA,
            dimensions: 0
        }
    }
    #[allow(non_snake_case)]
    pub fn CD ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::CD,
            dimensions: 0
        }
    }

    /// Returns true if this type implements Copy in Rust (cheap to duplicate).
    /// RE (f64), LO (bool), CM (Complex64) are Copy at dimension 0.
    /// All array types (dimensions > 0) are non-Copy (Vec<...>).
    pub fn is_copy(&self) -> bool {
        if self.dimensions > 0 {
            return false; // arrays are Vec<...>, not Copy
        }
        matches!(self.base_type, RosyBaseType::RE | RosyBaseType::LO | RosyBaseType::CM)
    }

    pub fn as_rust_type (&self) -> String {
        let base = match self.base_type {
            RosyBaseType::RE => "f64",
            RosyBaseType::ST => "String",
            RosyBaseType::LO => "bool",
            RosyBaseType::CM => "num_complex::Complex64",
            RosyBaseType::VE => "Vec<f64>",
            RosyBaseType::DA => "DA",
            RosyBaseType::CD => "CD",
        }.to_string();

        if self.dimensions == 0 {
            base
        } else {
            let mut result = base;
            for _ in 0..self.dimensions {
                result = format!("Vec<{}>", result);
            }
            result
        }
    }
}
impl TryFrom<&str> for RosyBaseType {
    type Error = anyhow::Error;
    fn try_from( value: &str ) -> Result<RosyBaseType, Self::Error> {
        match value {
            "RE" => Ok(RosyBaseType::RE),
            "ST" => Ok(RosyBaseType::ST),
            "LO" => Ok(RosyBaseType::LO),
            "CM" => Ok(RosyBaseType::CM),
            "VE" => Ok(RosyBaseType::VE),
            "DA" => Ok(RosyBaseType::DA),
            "CD" => Ok(RosyBaseType::CD),
            _ => Err(anyhow::anyhow!("Can't convert {} to a ROSY type", value)),
        }
    }
}