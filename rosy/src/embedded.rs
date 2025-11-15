use std::path::Path;
use anyhow::{Context, Result};

// Include the auto-generated embedded rosy_lib files
include!(concat!(env!("OUT_DIR"), "/embedded_rosy_lib.rs"));

/// Embedded main.rs template for generated projects
const MAIN_RS_TEMPLATE: &str = include_str!("../assets/output_template/main.rs");

/// Writes the vendored rosy_lib to the output directory
fn write_vendored_lib(output_dir: &Path) -> Result<()> {
    let lib_dir = output_dir.join("vendored/rosy_lib");
    
    for embedded_file in ROSY_LIB_FILES {
        // Determine the target path - rename mod.rs to lib.rs, put everything else in src/
        let target_path = if embedded_file.path == "mod.rs" {
            lib_dir.join("src/lib.rs")
        } else {
            lib_dir.join("src").join(embedded_file.path)
        };
        
        // Create parent directories
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
        
        // Transform the content: replace crate::rosy_lib:: with crate::
        // since when vendored, this IS the rosy_lib crate
        let mut transformed_content = embedded_file.content.replace("crate::rosy_lib::", "crate::");
        
        // Add warning suppressions to the lib.rs file
        if embedded_file.path == "mod.rs" {
            transformed_content = format!("#![allow(unused_imports)]\n#![allow(dead_code)]\n\n{}", transformed_content);
        }
        
        // Write the file content
        std::fs::write(&target_path, transformed_content)
            .with_context(|| format!("Failed to write file: {}", target_path.display()))?;
    }
    
    // Also need to write Cargo.toml for rosy_lib
    let lib_cargo_toml = r#"[package]
name = "rosy_lib"
version = "0.1.0"
edition = "2024"

[dependencies]
anyhow = "1.0"
mpi = "0.8"
bincode = "2.0"
dace = "0.2"
serial_test = "3.2"
num-complex = "0.4"
"#;
    
    std::fs::write(lib_dir.join("Cargo.toml"), lib_cargo_toml)
        .context("Failed to write vendored rosy_lib Cargo.toml")?;
    
    Ok(())
}

/// Generates a Cargo.toml for the output project
fn generate_cargo_toml() -> &'static str {
    r#"[package]
name = "rosy_output"
version = "0.1.0"
edition = "2024"

[dependencies]
anyhow = "1.0"
rosy_lib = { path = "./vendored/rosy_lib" }
mpi = "0.8"
dace = "0.2"
"#
}

/// Creates the output project structure in the specified directory
pub fn create_output_project(output_dir: &Path) -> Result<()> {
    // Create the directory structure
    std::fs::create_dir_all(output_dir.join("src"))
        .context("Failed to create output directory structure")?;
    
    // Write vendored rosy_lib
    write_vendored_lib(output_dir)
        .context("Failed to write vendored rosy_lib")?;
    
    // Write Cargo.toml
    std::fs::write(output_dir.join("Cargo.toml"), generate_cargo_toml())
        .context("Failed to write Cargo.toml template")?;
    
    // Write main.rs template
    std::fs::write(output_dir.join("src/main.rs"), MAIN_RS_TEMPLATE)
        .context("Failed to write main.rs template")?;
    
    Ok(())
}

/// Injects the transpiled code into the main.rs template
pub fn inject_code(transpiled_code: &str) -> Result<String> {
    // Split by injection markers
    let parts: Vec<&str> = MAIN_RS_TEMPLATE.split("// <INJECT_START>").collect();
    anyhow::ensure!(
        parts.len() == 2,
        "Expected exactly one '// <INJECT_START>' in main.rs template!"
    );
    
    let before_inject = parts[0];
    let parts: Vec<&str> = parts[1].split("// <INJECT_END>").collect();
    anyhow::ensure!(
        parts.len() == 2,
        "Expected exactly one '// <INJECT_END>' in main.rs template!"
    );
    
    let after_inject = parts[1];
    
    // Format the transpiled code with proper indentation
    let indented_code = transpiled_code
        .lines()
        .map(|line| format!("\t{}", line))
        .collect::<Vec<String>>()
        .join("\n");
    
    Ok(format!(
        "{}// <INJECT_START>\n{}\n\t// <INJECT_END>{}",
        before_inject,
        indented_code,
        after_inject
    ))
}
