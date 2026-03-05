# ROSY Language Support for Visual Studio Code

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
