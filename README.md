# Rosy 🌹

A modern Rust-based transpiler for the ROSY scientific programming language, designed for beam physics and differential algebra applications.

## What is Rosy?

Rosy transpiles ROSY source code into self-contained, native Rust executables. Rather than interpreting scripts, the generated binaries compile to optimized native code with Rust's memory safety guarantees and zero runtime dependencies.

For the **full language reference** — types, operators, statements, intrinsic functions, and examples — see the **[Rustdoc documentation](https://hiibolt.github.io/cosy-rs/rosy/index.html)**.

## Quick Start

### Prerequisites

**Using Nix Flakes (recommended):**

```bash
nix develop
```

This provides the complete development environment: Rust toolchain, MPI libraries, LLVM, and all system dependencies. No manual setup needed.

**Manual setup (without Nix):**

You will need:
- **Rust toolchain** (stable, edition 2024): [rustup.rs](https://rustup.rs/)
- **MPI implementation** (for `PLOOP` parallel loops):
  - Ubuntu/Debian: `sudo apt install libopenmpi-dev openmpi-bin`
  - Fedora: `sudo dnf install openmpi-devel`
  - macOS: `brew install open-mpi`
  - Arch: `sudo pacman -S openmpi`
- **LLVM/Clang** (for MPI bindings):
  - Ubuntu/Debian: `sudo apt install libclang-dev llvm-dev`
  - Fedora: `sudo dnf install clang-devel llvm-devel`
  - macOS: `brew install llvm`
  - Arch: `sudo pacman -S clang llvm`

### Building

```bash
cargo build --release
```

### Running a ROSY Script

```bash
# Run directly (binary stays in .rosy_output/)
rosy run examples/basic.rosy

# Build a standalone binary in the current directory
rosy build examples/basic.rosy

# Build with optimizations
rosy build examples/basic.rosy --release

# Custom output name
rosy build examples/basic.rosy -o my_program
```

## IDE Support

A VSCode extension with syntax highlighting is included. To install:

1. Copy the `rosy-vscode-extension/` folder to your VSCode extensions directory:
   - **Linux/macOS**: `~/.vscode/extensions/`
   - **Windows**: `%USERPROFILE%\.vscode\extensions\`
2. Reload VSCode

To regenerate the extension after grammar changes:
```bash
cargo run --bin generate_vscode_extension
```

## Language Documentation

The complete ROSY language reference is available as **Rustdoc**:

```bash
cargo doc --document-private-items --no-deps -p rosy --open
```

This includes:
- All 7 base types (`RE`, `ST`, `LO`, `CM`, `VE`, `DA`, `CD`) with descriptions
- Every operator with full type compatibility tables
- All intrinsic functions (`SIN`, `SQR`, `EXP`, `LENGTH`, etc.)
- Every statement (`VARIABLE`, `LOOP`, `IF`, `PROCEDURE`, `FUNCTION`, `FIT`, etc.)
- ROSY code examples throughout

The docs are also deployed at: **https://hiibolt.github.io/cosy-rs/rosy/index.html**

## Developing Rosy

### Development Environment

```bash
# Enter the Nix dev shell (provides MPI, LLVM, Rust, etc.)
nix develop

# Build the transpiler
cargo build

# Run tests via example scripts
rosy run examples/basic.rosy
rosy run examples/da_test.rosy
rosy run examples/vectors_arrays.rosy

# Build and view documentation
cargo doc --document-private-items --no-deps -p rosy --open
```

### Project Structure

| Directory | Purpose |
|-----------|---------|
| `rosy/src/` | Transpiler source (parser, AST, type resolution, code generation) |
| `rosy/src/rosy_lib/` | Embedded runtime library (operators, intrinsics, Taylor series, MPI) |
| `rosy/assets/rosy.pest` | PEG grammar defining ROSY syntax |
| `rosy/assets/output_template/` | Template for generated Rust projects |
| `rosy_ide_tools/` | VSCode extension generator |
| `examples/` | Example ROSY programs |

### Adding a New Expression

1. Read `manual.md` for the operator/function spec and type tables
2. Add grammar rule to `rosy/assets/rosy.pest`
3. Create AST struct in `rosy/src/program/expressions/`
4. Implement `FromRule`, `Transpile`, and `TranspileableExpr` traits
5. For operators: define `TypeRule` registry in `rosy/src/rosy_lib/operators/`
6. `cargo build` auto-generates test files from the registry
7. Validate against COSY INFINITY output — must match exactly

### Adding a New Statement

1. Read `manual.md` for the statement spec
2. Add grammar rule to `rosy/assets/rosy.pest`
3. Create AST struct in `rosy/src/program/statements/`
4. Add variant to `StatementEnum` and `Statement::from_rule`
5. Implement `FromRule` and `Transpile` traits
6. Add integration test to `examples/`

### Differences from COSY INFINITY

- `PLOOP` does not revert to `LOOP` behavior when `NP == 1`
- Rosy offers a `BREAK` statement for loop exit
- String literals use single quotes: `'hello'`

## License

See repository for license details.
