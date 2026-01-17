# Rosy

A modern Rust implementation of a transpiler for the ROSY programming language, designed for scientific computing and beam physics applications.

## Overview

Rosy is a complete transpiler toolchain that converts ROSY source code to executable Rust programs. ROSY is a scientific programming language originally developed for the COSY INFINITY beam physics simulation environment, featuring:

- **Strong static typing** with types like `RE` (real), `ST` (string), `LO` (logical), `CM` (complex), and `VE` (vector)
- **Procedures and functions** with parameter passing
- **Control flow constructs** including `IF/ELSE`, `LOOP`, and `WHILE`
- **Multi-dimensional arrays** with dynamic sizing
- **Built-in mathematical operations** optimized for scientific computing
- **Comprehensive type checking** at transpilation time

## Project Structure

The project is organized as a Rust workspace with several interconnected crates:

- **`rosy_transpiler/`** - Main transpiler binary that parses ROSY source and generates Rust code
- **`rosy_lib/`** - Runtime library providing ROSY data types and operations in Rust
- **`rosy_output/`** - Template target for generated Rust executables
- **`rosy_ide_tools/`** - Development tools including VSCode syntax highlighting extension

## Contributing: Adding New Language Features

### Adding a New Expression

1. Grammar definition added to `rosy/assets/rosy.pest` (either as `term` for primaries or `infix_op` for binary operators)
2. Infix operators added to Pratt parser precedence table in `rosy/src/ast.rs`
3. New struct created in `rosy/src/program/expressions/<name>.rs` with parsing logic
4. Module declaration added to `rosy/src/program/expressions/mod.rs`
5. Enum variant added to `ExprEnum`
6. Pratt parser mapping implemented (`map_primary` for terms, `map_infix` for operators)
7. Traits implemented: `TranspileWithType`, `TypeOf`, and `Transpile`
8. For operators: `TypeRule` registry defined in `rosy/src/rosy_lib/operators/<name>.rs` with test values
9. Build triggers codegen (`cargo build`) to generate test files
10. COSY/ROSY output comparison validates behavior matches reference implementation

### Adding a New Statement

1. Grammar rule added to `rosy/assets/rosy.pest` under `statement` production
2. New struct created in `rosy/src/program/statements/<name>.rs`
3. Module declaration added to `rosy/src/program/statements/mod.rs`
4. Enum variant added to `StatementEnum`
5. Pattern matching case added to `Statement::from_rule` in `mod.rs`
6. Traits implemented: `FromRule` and `Transpile`
7. Integration tests added to `examples/` directory
8. COSY/ROSY output comparison validates end-to-end behavior

### Test-Driven Development

All language features follow a strict TDD workflow:
- Test values embedded in `TypeRule` registries (for operators)
- `cargo build` auto-generates `.rosy` and `.fox` test scripts
- Transpile both ROSY and COSY INFINITY versions
- Output diff must be identical before merge (no approximations)
- Registry serves three purposes: type checking, runtime dispatch, and test generation

### Documenation, Help
This repository will be filled out as the language stabilizes. Until then, stay tuned :)