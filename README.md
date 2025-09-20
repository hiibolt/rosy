# COSY-RS

A Rust-based transpiler for the COSY programming language used in beam physics simulations.

## About COSY

COSY (COmputational SYstem) is a proprietary domain-specific language developed at Michigan State University (MSU) for beam physics and accelerator modeling. It's primarily used for:

- Charged particle beam dynamics calculations
- Accelerator design and optimization  
- High-order transfer map computations
- Beam optics simulations

This project implements a transpiler that converts COSY source code to Rust, allowing COSY programs to be compiled and executed using modern Rust tooling.

## Features

- **Parser**: Built with Pest parser generator using COSY grammar
- **AST Generation**: Converts parsed code into an Abstract Syntax Tree
- **Static Analysis**: Validates variable scope, function signatures, and type checking
- **Rust Transpilation**: Generates equivalent Rust code with COSY data types
- **Automatic Compilation**: Compiles the generated Rust code to native binaries
- **Execution**: Runs the compiled programs

## COSY Data Types

The transpiler supports COSY's specialized data types:
- **RE** (Real) - Floating-point numbers
- **ST** (String) - Text strings  
- **LO** (Logical) - Boolean values
- **CM** (Complex) - Complex numbers
- **VE** (Vector) - Arrays of real numbers

## Usage

1. Place your COSY source code in `inputs/basic.cosy`
2. Run the transpiler:
   ```bash
   cargo run
   ```

The transpiler performs these stages:
1. **Parsing** - Lexical and syntactic analysis of COSY code
2. **AST Generation** - Builds an intermediate representation
3. **Static Analysis** - Validates program correctness
4. **Transpilation** - Converts to Rust with COSY runtime
5. **Compilation** - Compiles Rust code to native binary
6. **Execution** - Runs the generated program

## Output Files

- `outputs/main.ast` - Generated AST for debugging
- `outputs/src/main.rs` - Transpiled Rust source code
- `outputs/target/release/rust` - Compiled executable

## COSY Language Features

### Variables
```cosy
VARIABLE X 8 ;          // Declare variable X with length 8
X := 42 ;               // Assignment
```

### Functions
```cosy
FUNCTION COMPUTE A B ;
    COMPUTE := A + B ;  // Function return value
ENDFUNCTION ;
```

### Procedures
```cosy
PROCEDURE PRINT_VALUE X ;
    WRITE 6 "Value: " X ;
ENDPROCEDURE ;
```

### Loops
```cosy
LOOP I 1 10 2 ;         // Loop from 1 to 10, step 2
    WRITE 6 I ;
ENDLOOP ;
```

### I/O Operations
```cosy
READ 5 X ;              // Read from stdin (unit 5)
WRITE 6 "Result: " X ;  // Write to stdout (unit 6)
```

### Built-in Functions
```cosy
Y := EXP(X) ;           // Exponential function
Z := CM(A & B) ;        // Convert to complex number
```

### Operators
- `+` - Addition
- `&` - Concatenation (creates vectors or strings)

## Example Program

```cosy
BEGIN ;
    FUNCTION QUADRATIC A B C X ;
        QUADRATIC := A * X * X + B * X + C ;
    ENDFUNCTION ;
    
    PROCEDURE SOLVE_EQUATION ;
        VARIABLE A 4 ;
        VARIABLE B 4 ;
        VARIABLE C 4 ;
        VARIABLE X 4 ;
        
        A := 1 ;
        B := -5 ;
        C := 6 ;
        X := 2 ;
        
        WRITE 6 "f(" X ") = " QUADRATIC(A,B,C,X) ;
    ENDPROCEDURE ;
    
    PROCEDURE RUN ;
        SOLVE_EQUATION ;
    ENDPROCEDURE ;
END ;
```

## Architecture

The transpiler uses a multi-stage compilation pipeline:

1. **Lexing/Parsing** - Pest parser processes COSY syntax
2. **AST Construction** - Builds typed abstract syntax tree
3. **Semantic Analysis** - Variable scoping and type validation
4. **Code Generation** - Emits Rust code with COSY runtime
5. **Native Compilation** - Rust compiler produces optimized binary

The generated Rust code includes a custom COSY runtime that implements the specialized data types and operations required for beam physics calculations.