# Rosy

A modern Rust implementation of a transpiler for the ROSY programming language, designed for scientific computing and beam physics applications.

## What is Rosy?

Rosy is a complete transpiler toolchain that converts ROSY source code into native executable Rust programs. ROSY is a scientific programming language originally developed for the COSY INFINITY beam physics simulation environment, now reimplemented as a modern transpiler with several key design decisions:

**Transpilation, Not Interpretation**: Rather than executing scripts directly, Rosy generates self-contained Rust programs that compile to native binaries. This provides:
- **Rust's memory safety guarantees** without runtime overhead
- **Performance comparable to hand-written Rust** for numerical computing
- **Zero runtime dependencies** - generated binaries are fully standalone
- **Compile-time type checking** catches errors before execution

**Registry-Based Type System**: All operators use a `TypeRule` registry as the single source of truth for:
- Type compatibility validation during transpilation
- Runtime dispatch to optimized implementations
- Automatic test generation for comprehensive validation

**Built for Scientific Computing**: Native support for:
- **Differential Algebra (DA/CD types)** for automatic differentiation and Taylor series
- **MPI parallelization** with built-in `PLOOP` constructs for distributed computing
- **Multi-dimensional arrays** with arbitrary dimensions `(RE ** 2 ** 3)` 
- **Complex numbers, vectors, and matrices** as first-class types
- **Strong type safety** with compile-time checking of all operations

**Test-Driven Development**: Every language feature is validated against COSY INFINITY's reference implementation through automated output comparison, ensuring behavioral compatibility.

## Language Features & Syntax

```rosy
BEGIN;
    {Function to add two numbers}
    FUNCTION (RE) ADD_TWO a (RE) b (RE);
        ADD_TWO := a + b;
    ENDFUNCTION;
    
    {Function to multiply and add}
    FUNCTION (RE) COMPUTE x (RE) y (RE);
        VARIABLE (RE) temp;
        temp := x * y;
        COMPUTE := temp + 10;
    ENDFUNCTION;
    
    {Procedure demonstrating conditionals}
    PROCEDURE CONDITIONAL_DEMO;
        VARIABLE (LO) is_true;
        VARIABLE (LO) is_false;
        
        is_true := TRUE;
        is_false := FALSE;
        
        IF is_true;
            WRITE 6 "First condition is TRUE";
        ELSEIF is_false;
            WRITE 6 "Second condition is TRUE";
        ELSE;
            WRITE 6 "Neither condition is TRUE";
        ENDIF;
        
        IF is_false;
            WRITE 6 "This should not print";
        ELSE;
            WRITE 6 "ELSE clause works!";
        ENDIF;
    ENDPROCEDURE;
    
    {Procedure demonstrating vectors and arrays}
    PROCEDURE VECTOR_DEMO;
        VARIABLE (VE) vec;
        VARIABLE (RE 2 3) matrix;
        
        {Concatenate values into vector}
        vec := 1 & 2 & 3 & 4 & 5;
        WRITE 6 "Vector: " ST(vec);
        
        {Access and assign array elements}
        matrix[1, 2] := 42;
        WRITE 6 "Matrix[1,2] = " ST(matrix[1, 2]);
    ENDPROCEDURE;
    
    {Procedure demonstrating loops}
    PROCEDURE LOOP_DEMO;
        VARIABLE (RE) sum;
        sum := 0;
        
        LOOP i 1 5;
            sum := sum + i;
            WRITE 6 "i = " ST(i) ", sum = " ST(sum);
        ENDLOOP;
    ENDPROCEDURE;
    
    {Main entry point}
    PROCEDURE RUN;
        VARIABLE (RE) x;
        VARIABLE (RE) y;
        VARIABLE (RE) result;
        
        x := 3;
        y := 4;
        
        result := ADD_TWO(x, y);
        WRITE 6 "Adding " ST(x) " + " ST(y) " = " ST(result);
        
        result := COMPUTE(x, y);
        WRITE 6 "Computing " ST(x) " * " ST(y) " + 10 = " ST(result);
        
        CONDITIONAL_DEMO;
        VECTOR_DEMO;
        LOOP_DEMO;
    ENDPROCEDURE;
    
    RUN;
END;
```

### Key Language Features

- **Strong Static Typing**: `(RE)` real, `(ST)` string, `(LO)` logical/boolean, `(CM)` complex, `(VE)` vector, `(DA)` differential algebra, `(CD)` complex DA
- **Multi-dimensional Arrays**: `(RE 2 3 4)` declares a 2×3×4 array of reals
- **Concatenation Operator**: `&` builds vectors from scalars: `1 & 2 & 3` creates a vector
- **Extraction Operator**: `|` extracts vector length for iteration
- **Type Conversion**: `ST()` converts to string, `CM()` to complex, `LO()` to logical
- **Procedures & Functions**: Procedures have no return value, functions return typed values
- **Comments**: `{This is a comment}` using curly braces
- **Output**: `WRITE 6 <expr>+` writes to stdout (unit 6)

## Installation

### Prerequisites

- **Rust toolchain** (1.70+): Install from [rustup.rs](https://rustup.rs/)
- **(Optional) Nix**: For reproducible development environment with MPI support

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/rosy.git
cd rosy

# Build the transpiler
cargo build --release

# The binary will be at target/release/rosy
# Optionally, install to your PATH
cargo install --path rosy
```

### Using Nix (Recommended for MPI features)

```bash
# Enter the development shell with all dependencies
nix develop

# Build and run normally
cargo build --release
```

The Nix environment provides MPI libraries and LLVM required for full functionality.

## Quick Start

### Running a ROSY Script

The quickest way to test a script is with `rosy run`:

```bash
# Run a script directly (compiled binary stays in .rosy_output/)
rosy run examples/basic.rosy

# Run with optimizations
rosy run examples/basic.rosy --release

# Use custom build directory
rosy run examples/basic.rosy -d /tmp/my_build

# Enable verbose logging
RUST_LOG=info rosy run examples/basic.rosy
```

### Building an Executable

To create a standalone binary:

```bash
# Build and copy binary to current directory
rosy build examples/basic.rosy
# Creates ./basic executable

# Specify output name
rosy build examples/basic.rosy -o my_program

# Build with optimizations (slower compile, faster execution)
rosy build examples/basic.rosy --release

# Custom build directory
rosy build examples/basic.rosy -d /tmp/build
```

### Workflow

1. **Write** your ROSY source code (`.rosy` extension)
2. **Transpile** with `rosy build script.rosy`
3. **Execute** the generated binary `./script`

The transpiler generates a complete Rust project in `.rosy_output/`, compiles it with cargo, and produces a self-contained executable.

## Examples

The `examples/` directory contains various demonstrations:

- **`basic.rosy`** - Functions, procedures, loops, and vector operations
- **`vectors_arrays.rosy`** - Multi-dimensional array indexing and manipulation
- **`if_statements.rosy`** - Conditional branching with IF/ELSEIF/ELSE
- **`da_test.rosy`** - Differential algebra for automatic differentiation
- **`ploop.rosy`** - Parallel loops with MPI distribution
- **`global_vars.rosy`** - Variable scoping and closure capture

Run any example with:
```bash
rosy run examples/<example>.rosy
```

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