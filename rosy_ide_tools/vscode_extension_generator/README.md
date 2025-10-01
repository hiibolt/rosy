# VSCode Extension Generator


## Building
To build:
```bash
cargo run --bin generate_vscode_extension -- --pest-file ./rosy.pest --output-dir ./rosy_ide_tools/rosy-vscode-extension
```

## Testing
Copy the extension to your extension directory:
```bash
cp -r rosy-rs/rosy_ide_tools/rosy-vscode-extension ~/.vscode/extensions/
```