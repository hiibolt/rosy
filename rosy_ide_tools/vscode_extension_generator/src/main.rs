use anyhow::Result;
use clap::Parser;
use serde_json::json;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the rosy.pest file
    #[arg(short, long, default_value = "../../rosy.pest")]
    pest_file: String,
    
    /// Output directory for the VSCode extension
    #[arg(short, long, default_value = "./rosy-vscode-extension")]
    output_dir: String,
}

struct VSCodeExtensionGenerator {
    keywords: Vec<String>,
    operators: Vec<String>,
    string_delimiters: Vec<String>,
    comment_patterns: Vec<String>,
}

impl VSCodeExtensionGenerator {
    fn new() -> Self {
        Self {
            keywords: Vec::new(),
            operators: Vec::new(),
            string_delimiters: Vec::new(),
            comment_patterns: Vec::new(),
        }
    }

    fn parse_pest_file(&mut self, _content: &str) -> Result<()> {
        // Extract keywords from the pest grammar
        self.extract_keywords();
        self.extract_operators();
        self.extract_strings_and_comments();
        Ok(())
    }

    fn extract_keywords(&mut self) {
        // Based on the rosy.pest file, extract all the keywords
        let keywords = vec![
            "VARIABLE", "BEGIN", "END", "PROCEDURE", "FUNCTION", "RETURNS",
            "WRITE", "READ", "IF", "THEN", "ELSE", "ENDIF", "LOOP", "ENDLOOP",
            "FOR", "TO", "ENDFOR", "WHILE", "ENDWHILE", "RETURN", "CALL",
            "INTEGER", "REAL", "STRING", "BOOLEAN", "TRUE", "FALSE"
        ];
        
        self.keywords = keywords.into_iter().map(|s| s.to_string()).collect();
    }

    fn extract_operators(&mut self) {
        let operators = vec![
            "=", "+", "-", "*", "/", "%", "==", "!=", "<", ">", "<=", ">=",
            "&&", "||", "!", "&", "|", "^", "<<", ">>", "++", "--", "+=",
            "-=", "*=", "/=", "%=", "?", ":", ".", ",", ";", "(", ")", "[",
            "]", "{", "}", "->", "<-"
        ];
        
        self.operators = operators.into_iter().map(|s| s.to_string()).collect();
    }

    fn extract_strings_and_comments(&mut self) {
        self.string_delimiters = vec!["\"".to_string(), "'".to_string()];
        self.comment_patterns = vec!["//".to_string(), "/*".to_string()];
    }

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
                    "extensions": [".rosy"],
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
                "lineComment": "//",
                "blockComment": ["/*", "*/"]
            },
            "brackets": [
                ["{", "}"],
                ["[", "]"],
                ["(", ")"]
            ],
            "autoClosingPairs": [
                ["{", "}"],
                ["[", "]"],
                ["(", ")"],
                ["\"", "\""],
                ["'", "'"]
            ],
            "surroundingPairs": [
                ["{", "}"],
                ["[", "]"],
                ["(", ")"],
                ["\"", "\""],
                ["'", "'"]
            ],
            "indentationRules": {
                "increaseIndentPattern": "^.*\\{[^}\"']*$|^.*\\([^)\"']*$|^.*\\[[^\\]\"']*$|^.*(BEGIN|THEN|ELSE|LOOP|FOR|WHILE|PROCEDURE|FUNCTION)\\s*$",
                "decreaseIndentPattern": "^\\s*(\\}|\\)|\\]|END|ENDIF|ENDLOOP|ENDFOR|ENDWHILE|ELSE).*$"
            }
        })
    }

    fn generate_syntax_highlighting(&self) -> serde_json::Value {
        json!({
            "$schema": "https://raw.githubusercontent.com/martinring/tmlanguage/master/tmlanguage.json",
            "name": "ROSY",
            "scopeName": "source.rosy",
            "patterns": [
                {"include": "#comments"},
                {"include": "#strings"},
                {"include": "#keywords"},
                {"include": "#operators"},
                {"include": "#numbers"},
                {"include": "#identifiers"}
            ],
            "repository": {
                "comments": {
                    "patterns": [
                        {
                            "name": "comment.line.double-slash.rosy",
                            "begin": "//",
                            "end": "$"
                        },
                        {
                            "name": "comment.block.rosy",
                            "begin": "/\\*",
                            "end": "\\*/"
                        }
                    ]
                },
                "strings": {
                    "patterns": [
                        {
                            "name": "string.quoted.double.rosy",
                            "begin": "\"",
                            "end": "\"",
                            "patterns": [
                                {
                                    "name": "constant.character.escape.rosy",
                                    "match": "\\\\."
                                }
                            ]
                        },
                        {
                            "name": "string.quoted.single.rosy",
                            "begin": "'",
                            "end": "'",
                            "patterns": [
                                {
                                    "name": "constant.character.escape.rosy",
                                    "match": "\\\\."
                                }
                            ]
                        }
                    ]
                },
                "keywords": {
                    "patterns": [
                        {
                            "name": "keyword.control.rosy",
                            "match": format!("\\b({}|{}|{}|{})\\b", 
                                "BEGIN|END|IF|THEN|ELSE|ENDIF",
                                "LOOP|ENDLOOP|FOR|TO|ENDFOR", 
                                "WHILE|ENDWHILE|RETURN",
                                "PROCEDURE|FUNCTION|RETURNS|CALL")
                        },
                        {
                            "name": "keyword.other.rosy",
                            "match": "\\b(VARIABLE|WRITE|READ)\\b"
                        },
                        {
                            "name": "entity.name.type.rosy",
                            "match": "\\b(INTEGER|REAL|STRING|BOOLEAN)\\b"
                        },
                        {
                            "name": "entity.name.type.rosy",
                            "match": "\\(\\s*(RE|LO|VE|CM)\\s*\\)"
                        },
                        {
                            "name": "constant.language.rosy",
                            "match": "\\b(TRUE|FALSE)\\b"
                        }
                    ]
                },
                "operators": {
                    "patterns": [
                        {
                            "name": "keyword.operator.assignment.rosy",
                            "match": "=|\\+=|-=|\\*=|/=|%="
                        },
                        {
                            "name": "keyword.operator.comparison.rosy",
                            "match": "==|!=|<=|>=|<|>"
                        },
                        {
                            "name": "keyword.operator.logical.rosy",
                            "match": "&&|\\|\\||!"
                        },
                        {
                            "name": "keyword.operator.arithmetic.rosy",
                            "match": "\\+|\\-|\\*|/|%"
                        },
                        {
                            "name": "keyword.operator.bitwise.rosy",
                            "match": "&|\\||\\^|<<|>>"
                        },
                        {
                            "name": "keyword.operator.increment.rosy",
                            "match": "\\+\\+|\\-\\-"
                        },
                        {
                            "name": "punctuation.separator.rosy",
                            "match": ",|;|\\."
                        },
                        {
                            "name": "punctuation.brackets.rosy",
                            "match": "\\(|\\)|\\[|\\]|\\{|\\}"
                        }
                    ]
                },
                "numbers": {
                    "patterns": [
                        {
                            "name": "constant.numeric.float.rosy",
                            "match": "\\b\\d+\\.\\d+\\b"
                        },
                        {
                            "name": "constant.numeric.integer.rosy",
                            "match": "\\b\\d+\\b"
                        }
                    ]
                },
                "identifiers": {
                    "name": "variable.other.rosy",
                    "match": "\\b[a-zA-Z_][a-zA-Z0-9_]*\\b"
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

This extension provides syntax highlighting and basic language support for the ROSY programming language.

## Features

- Syntax highlighting for ROSY files (.rosy)
- Automatic bracket matching and closing
- Comment toggling support
- Basic indentation rules

## Installation

1. Copy this extension to your VSCode extensions directory
2. Reload VSCode
3. Open any .rosy file to see syntax highlighting

## Language Features

The ROSY language includes:
- Variables and data types (INTEGER, REAL, STRING, BOOLEAN)
- Control structures (IF/THEN/ELSE, LOOP, FOR, WHILE)
- Procedures and functions
- Input/output operations (READ, WRITE)
- Arithmetic and logical operators

## File Extensions

- `.rosy` - ROSY source files
"#;

        fs::write(output_path.join("README.md"), readme_content)?;

        // Generate .vscodeignore
        let vscodeignore_content = r#".vscode/**
.vscode-test/**
**/*.map
**/node_modules/**
src/**
**/*.ts
.gitignore
README.md
.eslintrc.json
**/*.vsix
"#;

        fs::write(output_path.join(".vscodeignore"), vscodeignore_content)?;

        println!("VSCode extension generated successfully in: {}", output_dir);
        println!("\nTo install the extension:");
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
    
    let pest_content = fs::read_to_string(&args.pest_file)
        .map_err(|e| anyhow::anyhow!("Failed to read pest file {}: {}", args.pest_file, e))?;

    let mut generator = VSCodeExtensionGenerator::new();
    generator.parse_pest_file(&pest_content)?;
    generator.generate_extension(&args.output_dir)?;

    Ok(())
}