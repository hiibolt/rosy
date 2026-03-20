# Rosy 🌹

A modern Rust-based transpiler for the ROSY scientific programming language, designed for beam physics and differential algebra applications.

## What is Rosy?

Rosy transpiles ROSY source code into self-contained, native Rust executables. Rather than interpreting scripts, the generated binaries compile to optimized native code with Rust's memory safety guarantees and zero runtime dependencies.

For the **full language reference** — types, operators, statements, intrinsic functions, and examples — see the **[Rustdoc documentation](https://hiibolt.github.io/cosy-rs/rosy/index.html)**.

## Installation

### From source (recommended)

Requires the [Rust toolchain](https://rustup.rs/) (stable, edition 2024):

```bash
git clone https://github.com/hiibolt/rosy.git
cd rosy
cargo install --path rosy
```

This installs the `rosy` binary to `~/.cargo/bin/` (which should already be in your `PATH` if you have Rust installed).

To update:

```bash
git pull && cargo install --path rosy
```

### From GitHub Releases

Prebuilt binaries for Linux (x86_64) and macOS (x86_64, aarch64) are available on the [Releases page](https://github.com/hiibolt/rosy/releases/latest). Download the binary for your platform and place it somewhere in your `PATH`.

### Using Nix Flakes

```bash
nix develop
```

This provides the complete development environment: Rust toolchain and all system dependencies.

## Quick Start

```bash
# Run a Rosy script directly
rosy run examples/basic.rosy

# Build a standalone binary in the current directory
rosy build examples/basic.rosy

# Build with optimizations
rosy build examples/basic.rosy --release

# Custom output name
rosy build examples/basic.rosy -o my_program
```

### MPI support (`PLOOP`)

Programs that use the `PLOOP` construct require an MPI implementation and LLVM/Clang at compile time. The Rosy transpiler itself does not require these — they are only needed when compiling generated programs that use parallel features.

- **MPI implementation**:
  - Ubuntu/Debian: `sudo apt install libopenmpi-dev openmpi-bin`
  - Fedora: `sudo dnf install openmpi-devel`
  - macOS: `brew install open-mpi`
  - Arch: `sudo pacman -S openmpi`
- **LLVM/Clang** (for MPI bindings):
  - Ubuntu/Debian: `sudo apt install libclang-dev llvm-dev`
  - Fedora: `sudo dnf install clang-devel llvm-devel`
  - macOS: `brew install llvm`
  - Arch: `sudo pacman -S clang llvm`

### NIU Metis Supercomputer

On the NIU Metis supercomputer, load the MPI module before running any Rosy binary that uses MPI features (e.g., `PLOOP`):

```bash
module load openmpi/openmpi-5.0.7-gcc-14.2.0-cuda-12.8
```

This is only required for *running* MPI-compiled binaries, not for the Rosy transpiler itself.

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
3. Create AST struct in `rosy/src/program/expressions/` (e.g. `MyExpr`)
4. Add a variant to `ExprEnum` in `rosy/src/program/expressions/mod.rs`
5. Wire it into `Expr::from_rule` (for primaries) or `map_infix` (for binary operators)
6. Implement three traits on your struct:

   **`Transpile`** — code generation:
   ```rust
   impl Transpile for MyExpr {
       fn transpile(&self, context: &mut TranspilationInputContext)
           -> Result<TranspilationOutput, Vec<Error>> { /* ... */ }
   }
   ```

   **`TranspileableExpr`** — type inference integration:
   ```rust
   impl TranspileableExpr for MyExpr {
       // Required: return this expression's type given the current context
       fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> { /* ... */ }

       // Optional: recurse into child expressions for function call discovery.
       // Only needed if this expression contains sub-expressions.
       // Default returns None (leaf node, nothing to recurse into).
       fn discover_expr_function_calls(&self, resolver: &mut TypeResolver, ctx: &ScopeContext)
           -> Option<Result<()>> { /* ... */ }

       // Optional: build an ExprRecipe for type inference.
       // Literals return ExprRecipe::Literal(...), binary ops return ExprRecipe::BinaryOp {...},
       // variables return ExprRecipe::Variable(slot). Default returns None (Unknown).
       fn build_expr_recipe(&self, resolver: &TypeResolver, ctx: &ScopeContext,
           deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> { /* ... */ }
   }
   ```

7. For operators: define `TypeRule` registry in `rosy/src/rosy_lib/operators/`
8. `cargo build` auto-generates test files from the registry
9. Validate against COSY INFINITY output — must match exactly

### Adding a New Statement

1. Read `manual.md` for the statement spec
2. Add grammar rule to `rosy/assets/rosy.pest`
3. Create AST struct in `rosy/src/program/statements/` (e.g. `MyStatement`)
4. Add a variant to `StatementEnum` in `rosy/src/program/statements/mod.rs`
5. Wire it into `Statement::from_rule`
6. Implement three traits on your struct:

   **`Transpile`** — code generation:
   ```rust
   impl Transpile for MyStatement {
       fn transpile(&self, context: &mut TranspilationInputContext)
           -> Result<TranspilationOutput, Vec<Error>> { /* ... */ }
   }
   ```

   **`TranspileableStatement`** — type inference integration (all methods optional):
   ```rust
   impl TranspileableStatement for MyStatement {
       // Register type slots (variables, args, return types) into the dependency graph.
       // Implement for declarations (VARIABLE, FUNCTION, PROCEDURE).
       fn register_declaration(&self, resolver: &mut TypeResolver, ctx: &mut ScopeContext,
           source_location: SourceLocation) -> Option<Result<()>> { /* ... */ }

       // Discover dependencies between type slots (e.g. assignments, call sites).
       // Implement for statements that assign values or call functions/procedures.
       fn discover_dependencies(&self, resolver: &mut TypeResolver, ctx: &mut ScopeContext,
           source_location: SourceLocation) -> Option<Result<()>> { /* ... */ }

       // Apply resolved types back to the AST after type inference completes.
       // Implement for statements that contain a body (LOOP, IF, FUNCTION, etc.)
       // so the resolver can recurse into child statements.
       fn apply_resolved_types(&mut self, resolver: &TypeResolver,
           current_scope: &[String]) -> Option<Result<()>> { /* ... */ }
   }
   ```

   For simple statements with no type inference involvement, an empty impl suffices:
   ```rust
   impl TranspileableStatement for MyStatement {}
   ```

7. Add integration test to `examples/`

### Differences from COSY INFINITY

- `PLOOP` does not revert to `LOOP` behavior when `NP == 1`
- Rosy offers a `BREAK` statement for loop exit
- String literals use single quotes: `'hello'`

## License

See repository for license details.
