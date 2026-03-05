use anyhow::Result;
use clap::Parser;
use serde_json::json;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the rosy.pest file
    #[arg(short, long, default_value = "./rosy/assets/rosy.pest")]
    pest_file: String,
    
    /// Output directory for the VSCode extension
    #[arg(short, long, default_value = "./rosy-vscode-extension")]
    output_dir: String,
}

struct VSCodeExtensionGenerator;

impl VSCodeExtensionGenerator {
    fn generate_package_json(&self) -> serde_json::Value {
        json!({
            "name": "rosy-language-support",
            "displayName": "ROSY Language Support",
            "description": "Syntax highlighting for the ROSY programming language",
            "version": "0.1.0",
            "engines": {
                "vscode": "^1.74.0"
            },
            "categories": ["Programming Languages"],
            "contributes": {
                "languages": [{
                    "id": "rosy",
                    "aliases": ["ROSY", "rosy"],
                    "extensions": [".rosy", ".fox"],
                    "configuration": "./language-configuration.json"
                }],
                "grammars": [{
                    "language": "rosy",
                    "scopeName": "source.rosy",
                    "path": "./syntaxes/rosy.tmLanguage.json"
                }]
            }
        })
    }

    fn generate_language_configuration(&self) -> serde_json::Value {
        json!({
            "comments": {
                "blockComment": ["{", "}"]
            },
            "brackets": [
                ["(", ")"],
                ["[", "]"]
            ],
            "autoClosingPairs": [
                { "open": "(", "close": ")" },
                { "open": "[", "close": "]" },
                { "open": "{", "close": "}" },
                { "open": "'", "close": "'", "notIn": ["string"] }
            ],
            "surroundingPairs": [
                ["(", ")"],
                ["[", "]"],
                ["{", "}"],
                ["'", "'"]
            ],
            "folding": {
                "markers": {
                    "start": "^\\s*(BEGIN|IF|LOOP|PLOOP|WHILE|PROCEDURE|FUNCTION|FIT)\\b",
                    "end": "^\\s*(END|ENDIF|ENDLOOP|ENDPLOOP|ENDWHILE|ENDPROCEDURE|ENDFUNCTION|ENDFIT)\\b"
                }
            },
            "indentationRules": {
                "increaseIndentPattern": "^\\s*(BEGIN|IF|ELSEIF|ELSE|LOOP|PLOOP|WHILE|PROCEDURE|FUNCTION|FIT)\\b",
                "decreaseIndentPattern": "^\\s*(END|ENDIF|ENDLOOP|ENDPLOOP|ENDWHILE|ENDPROCEDURE|ENDFUNCTION|ENDFIT|ELSEIF|ELSE)\\b"
            },
            "wordPattern": "[a-zA-Z_][a-zA-Z0-9_]*"
        })
    }

    fn generate_syntax_highlighting(&self) -> serde_json::Value {
        json!({
            "$schema": "https://raw.githubusercontent.com/martinring/tmlanguage/master/tmlanguage.json",
            "name": "ROSY",
            "scopeName": "source.rosy",
            "patterns": [
                { "include": "#comments" },
                { "include": "#strings" },
                { "include": "#constants" },
                { "include": "#types" },
                { "include": "#control-keywords" },
                { "include": "#declaration-keywords" },
                { "include": "#io-keywords" },
                { "include": "#da-keywords" },
                { "include": "#intrinsic-functions" },
                { "include": "#assignment-operator" },
                { "include": "#operators" },
                { "include": "#numbers" },
                { "include": "#punctuation" },
                { "include": "#identifiers" }
            ],
            "repository": {
                "comments": {
                    "patterns": [{
                        "name": "comment.block.rosy",
                        "begin": "\\{",
                        "end": "\\}",
                        "patterns": [{
                            "name": "comment.block.rosy",
                            "begin": "\\{",
                            "end": "\\}"
                        }]
                    }]
                },
                "strings": {
                    "patterns": [{
                        "name": "string.quoted.single.rosy",
                        "begin": "'",
                        "end": "'",
                        "patterns": [{
                            "name": "constant.character.escape.rosy",
                            "match": "''"
                        }]
                    }]
                },
                "constants": {
                    "patterns": [{
                        "name": "constant.language.boolean.rosy",
                        "match": "\\b(TRUE|FALSE)\\b"
                    }]
                },
                "types": {
                    "patterns": [
                        {
                            "comment": "Type annotation in parentheses: (RE), (VE ** 2), etc.",
                            "match": "\\(\\s*(RE|ST|LO|CM|VE|DA|CD)\\b",
                            "captures": {
                                "1": { "name": "entity.name.type.rosy" }
                            }
                        },
                        {
                            "comment": "Dimension operator in type annotations",
                            "name": "keyword.operator.dimension.rosy",
                            "match": "\\*\\*"
                        }
                    ]
                },
                "control-keywords": {
                    "patterns": [{
                        "name": "keyword.control.rosy",
                        "match": "\\b(BEGIN|END|IF|ELSEIF|ELSE|ENDIF|LOOP|ENDLOOP|PLOOP|ENDPLOOP|WHILE|ENDWHILE|BREAK|FIT|ENDFIT|NOT)\\b"
                    }]
                },
                "declaration-keywords": {
                    "patterns": [{
                        "name": "keyword.declaration.rosy",
                        "match": "\\b(VARIABLE|PROCEDURE|ENDPROCEDURE|FUNCTION|ENDFUNCTION)\\b"
                    }]
                },
                "io-keywords": {
                    "patterns": [{
                        "name": "keyword.other.io.rosy",
                        "match": "\\b(WRITE|WRITEB|READ|READB|OPENF|OPENFB|CLOSEF)\\b"
                    }]
                },
                "da-keywords": {
                    "patterns": [{
                        "name": "keyword.other.da.rosy",
                        "match": "\\b(OV|DAINI|DAPRV|DAREV)\\b"
                    }]
                },
                "intrinsic-functions": {
                    "patterns": [{
                        "name": "support.function.builtin.rosy",
                        "match": "\\b(SIN|TAN|SQR|EXP|LENGTH|VMAX|LST|LCM|LCD|DA|CD|CM|ST|LO)\\b(?=\\s*\\()"
                    }]
                },
                "assignment-operator": {
                    "patterns": [{
                        "name": "keyword.operator.assignment.rosy",
                        "match": ":="
                    }]
                },
                "operators": {
                    "patterns": [
                        {
                            "name": "keyword.operator.comparison.rosy",
                            "match": "<=|>=|<>|<|>|="
                        },
                        {
                            "name": "keyword.operator.arithmetic.rosy",
                            "match": "\\+|-|\\*|/"
                        },
                        {
                            "name": "keyword.operator.concatenation.rosy",
                            "match": "&"
                        },
                        {
                            "name": "keyword.operator.extraction.rosy",
                            "match": "\\|"
                        },
                        {
                            "name": "keyword.operator.derivation.rosy",
                            "match": "%"
                        },
                        {
                            "name": "keyword.operator.power.rosy",
                            "match": "\\^"
                        }
                    ]
                },
                "numbers": {
                    "patterns": [
                        {
                            "name": "constant.numeric.float.rosy",
                            "match": "\\b\\d+\\.\\d+([eE][+-]?\\d+)?\\b"
                        },
                        {
                            "name": "constant.numeric.scientific.rosy",
                            "match": "\\b\\d+[eE][+-]?\\d+\\b"
                        },
                        {
                            "name": "constant.numeric.integer.rosy",
                            "match": "\\b\\d+\\b"
                        }
                    ]
                },
                "punctuation": {
                    "patterns": [
                        {
                            "name": "punctuation.terminator.rosy",
                            "match": ";"
                        },
                        {
                            "name": "punctuation.separator.comma.rosy",
                            "match": ","
                        }
                    ]
                },
                "identifiers": {
                    "patterns": [{
                        "name": "variable.other.rosy",
                        "match": "\\b[a-zA-Z_][a-zA-Z0-9_]*\\b"
                    }]
                }
            }
        })
    }

    fn generate_extension(&self, output_dir: &str) -> Result<()> {
        let output_path = Path::new(output_dir);
        
        // Create directory structure
        fs::create_dir_all(output_path)?;
        fs::create_dir_all(output_path.join("syntaxes"))?;

        // Generate package.json
        let package_json = self.generate_package_json();
        fs::write(
            output_path.join("package.json"),
            serde_json::to_string_pretty(&package_json)?
        )?;

        // Generate language-configuration.json
        let lang_config = self.generate_language_configuration();
        fs::write(
            output_path.join("language-configuration.json"),
            serde_json::to_string_pretty(&lang_config)?
        )?;

        // Generate syntax highlighting
        let syntax_highlighting = self.generate_syntax_highlighting();
        fs::write(
            output_path.join("syntaxes/rosy.tmLanguage.json"),
            serde_json::to_string_pretty(&syntax_highlighting)?
        )?;

        // Generate README
        let readme_content = r#"# ROSY Language Support for Visual Studio Code

Syntax highlighting and editor support for the ROSY programming language.

## Features

- Full syntax highlighting for `.rosy` and `.fox` files
- Comment toggling (`{...}` block comments)
- Bracket matching and auto-closing
- Code folding for control structures
- Indentation rules

## Installation

Copy the entire extension folder to your VSCode extensions directory:

- **Linux**: `~/.vscode/extensions/`
- **macOS**: `~/.vscode/extensions/`
- **Windows**: `%USERPROFILE%\.vscode\extensions\`

Then reload VSCode. Open any `.rosy` file to see syntax highlighting.

## Regenerating

This extension is auto-generated from the ROSY grammar. To regenerate:

```bash
cargo run --bin generate_vscode_extension
```
"#;

        fs::write(output_path.join("README.md"), readme_content)?;

        println!("\nVSCode extension generated successfully in: {}\n", output_dir);
        println!("To install the extension:");
        println!("1. Copy the entire '{}' folder to your VSCode extensions directory:", output_dir);
        println!("   - Linux: ~/.vscode/extensions/");
        println!("   - macOS: ~/.vscode/extensions/");
        println!("   - Windows: %USERPROFILE%\\.vscode\\extensions\\");
        println!("2. Reload VSCode");
        println!("3. Open any .rosy file to see syntax highlighting");

        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // We still accept the pest file path for future use,
    // but currently keywords are defined statically to match the grammar.
    if !Path::new(&args.pest_file).exists() {
        eprintln!("Warning: pest file '{}' not found, using built-in definitions", args.pest_file);
    }

    let generator = VSCodeExtensionGenerator;
    generator.generate_extension(&args.output_dir)?;

    Ok(())
}
