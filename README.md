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

### Documenation, Help
This repository will be filled out as the language stabilizes. Until then, stay tuned :)