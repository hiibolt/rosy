---
name: cosy-reference
description: COSY INFINITY type system, operators, statements, and intrinsic functions reference. Use when implementing or reviewing ROSY language constructs.
user-invocable: false
---

# COSY INFINITY Quick Reference

## Type System

| COSY Type | Rust Type | Literal Example | Notes |
|-----------|-----------|-----------------|-------|
| RE | f64 | `3.14`, `-7` | Default numeric type |
| ST | String | `'hello'` | Single-quoted in COSY |
| LO | bool | `TRUE`, `FALSE` | Logical |
| CM | Complex64 | `CM(1&2)` = 1+2i | Via `&` concat for real/imag |
| VE | Vec\<f64\> | `1&2&3` | Built with `&` operator |
| DA | taylor::DA | `DA(1)` | Taylor series, requires DAINI |
| CD | taylor::CD | `CD(1)` | Complex Taylor series |

## Operator Precedence (lowest to highest)

| Priority | Operators | Associativity | Types |
|----------|-----------|---------------|-------|
| 2 | `&` `=` `#` `<` `>` `<=` `>=` | Left | concat, comparison |
| 3 | `+` `-` | Left | RE,CM,VE,DA,CD; LO+LO=OR |
| 4 | `*` `/` | Left | RE,CM,VE,DA,CD; LO*LO=AND |
| 5 | `^` | Right | RE,CM,DA,CD |
| 6 | `\|` `%` | Left | extract, derive |
| unary | `-` `!` | Prefix | negation, logical NOT |

## Implemented Operators (13)

Add(+), Sub(-), Mult(*), Div(/), Pow(^), Concat(&), Extract(\|), Derive(%),
Eq(=), Neq(#/<>/!=), Lt(<), Gt(>), Lte(<=), Gte(>=), Not(!)

## Implemented Statements (22)

**Core:** VARIABLE, :=, IF/ELSEIF/ELSE/ENDIF, LOOP/ENDLOOP, WHILE/ENDWHILE,
PLOOP/ENDPLOOP, PROCEDURE/ENDPROCEDURE, FUNCTION/ENDFUNCTION, BREAK,
procedure call, function call

**I/O:** WRITE, WRITEB, READ, READB, OPENF, OPENFB, CLOSEF

**DA:** OV/DAINI, DAPRV, DAREV

**Math:** FIT/ENDFIT

## Implemented Intrinsic Functions (11)

SIN, TAN, EXP, SQR, LENGTH, VMAX, CM(), ST(), LO(), LST, LCM, LCD

## Not Yet Implemented (from manual.md)

| Construct | Type | Notes |
|-----------|------|-------|
| COS | intrinsic function | Cosine -- high priority |
| ABS | intrinsic function | Absolute value -- high priority |
| LOG/LN | intrinsic function | Logarithm -- high priority |
| SQRT | intrinsic function | Alias for SQR in some contexts |
| ASIN/ATAN | intrinsic function | Inverse trig |
| RE() | conversion function | Convert to real |
| NORMA | intrinsic function | Norm |
| ER | intrinsic function | Error function |
| ISRT | intrinsic function | Inverse square root |
| POLVAL | intrinsic procedure | Polynomial evaluation |
| MAP | intrinsic procedure | Map operation |
| VELSET/VELGET | intrinsic procedure | Vector element access |

## COSY vs ROSY Differences

- PLOOP: COSY reverts to LOOP when NP==1; ROSY does not
- BREAK: ROSY extension, not in COSY
- Strings: COSY uses `'single'`; ROSY supports both `'single'` and `"double"`
- VARIABLE: COSY requires memory size param; ROSY omits it (--cosy-syntax restores it)
- Comments: Both use `{curly braces}`
