---
name: migration-mapping
description: Complete mapping of COSY INFINITY constructs to their Rosy equivalents with implementation status and file paths. Use when checking what exists and what needs implementing.
user-invocable: false
---

# COSY -> ROSY Migration Mapping

## Implemented Operators

| COSY Syntax | Rosy ExprEnum | Registry File | Type Combos |
|-------------|---------------|---------------|-------------|
| `+` | Add | `rosy/src/rosy_lib/operators/add.rs` | RE,CM,VE,DA,CD (20 combos) |
| `-` | Sub | `rosy/src/rosy_lib/operators/sub.rs` | RE,CM,VE,DA,CD |
| `*` | Mult | `rosy/src/rosy_lib/operators/mult.rs` | RE,CM,VE,DA,CD; LO*LO=AND |
| `/` | Div | `rosy/src/rosy_lib/operators/div.rs` | RE,CM,DA,CD |
| `^` | Pow | `rosy/src/rosy_lib/operators/pow.rs` | RE,CM,DA,CD (right-assoc) |
| `&` | Concat | `rosy/src/rosy_lib/operators/concat.rs` | Builds VE from scalars |
| `\|` | Extract | `rosy/src/rosy_lib/operators/extract.rs` | VE\|RE->RE, DA\|RE->RE |
| `%` | Derive | `rosy/src/rosy_lib/intrinsics/derive.rs` | DA%RE->DA, CD%RE->CD |
| `=`/`==` | Eq | `rosy/src/rosy_lib/operators/eq.rs` | RE,ST,LO,CM |
| `#`/`<>`/`!=` | Neq | `rosy/src/rosy_lib/operators/neq.rs` | RE,ST,LO,CM |
| `<` | Lt | `rosy/src/rosy_lib/operators/lt.rs` | RE |
| `>` | Gt | `rosy/src/rosy_lib/operators/gt.rs` | RE |
| `<=` | Lte | `rosy/src/rosy_lib/operators/lte.rs` | RE |
| `>=` | Gte | `rosy/src/rosy_lib/operators/gte.rs` | RE |
| `!`/NOT | Not | `rosy/src/rosy_lib/operators/not.rs` | LO (unary) |

## Implemented Intrinsic Functions

| COSY | Rosy ExprEnum | Registry File | Input Types |
|------|---------------|---------------|-------------|
| `SIN(x)` | Sin | `rosy/src/rosy_lib/intrinsics/sin.rs` | RE,CM,VE,DA |
| `TAN(x)` | Tan | `rosy/src/rosy_lib/intrinsics/tan.rs` | RE,CM,VE,DA |
| `EXP(x)` | Exp | `rosy/src/rosy_lib/intrinsics/exp.rs` | RE,CM,VE,DA |
| `SQR(x)` | Sqr | `rosy/src/rosy_lib/intrinsics/sqr.rs` | RE,CM,VE,DA |
| `LENGTH(x)` | Length | `rosy/src/rosy_lib/intrinsics/length.rs` | ST,VE |
| `VMAX(x)` | Vmax | `rosy/src/rosy_lib/intrinsics/vmax.rs` | VE |
| `CM(x)` | Complex | `rosy/src/rosy_lib/intrinsics/cm.rs` | RE->CM |
| `ST(x)` | StringConvert | `rosy/src/rosy_lib/intrinsics/st.rs` | RE,CM,LO,VE,DA,CD->ST |
| `LO(x)` | LogicalConvert | `rosy/src/rosy_lib/intrinsics/lo.rs` | RE->LO |
| `LST(x)` | Lst | `rosy/src/rosy_lib/intrinsics/mem_size.rs` | ST->RE (COSY compat) |
| `LCM(x)` | Lcm | `rosy/src/rosy_lib/intrinsics/mem_size.rs` | CM->RE (COSY compat) |
| `LCD(x)` | Lcd | `rosy/src/rosy_lib/intrinsics/mem_size.rs` | DA->RE (COSY compat) |

## Implemented Statements

| COSY Syntax | Rosy StatementEnum | AST File |
|-------------|-------------------|----------|
| `VARIABLE (type) name dims;` | VarDecl | `statements/core/var_decl.rs` |
| `name := expr;` | Assign | `statements/core/assign.rs` |
| `IF cond; ... ENDIF;` | If | `statements/core/if.rs` |
| `LOOP i start end [step]; ... ENDLOOP;` | Loop | `statements/core/loop.rs` |
| `WHILE cond; ... ENDWHILE;` | WhileLoop | `statements/core/while_loop.rs` |
| `PLOOP i start end; ... ENDPLOOP unit out;` | PLoop | `statements/core/ploop.rs` |
| `PROCEDURE name args; ... ENDPROCEDURE;` | Procedure | `statements/core/procedure.rs` |
| `FUNCTION [type] name args; ... ENDFUNCTION;` | Function | `statements/core/function.rs` |
| `name expr*;` (procedure call) | ProcedureCall | `statements/core/procedure_call.rs` |
| `name(expr, ...);` (function call) | FunctionCall | `statements/core/function_call.rs` |
| `BREAK;` | Break | `statements/core/break_statement.rs` |
| `WRITE unit expr+;` | Write | `statements/io/write.rs` |
| `WRITEB unit expr+;` | Writeb | `statements/io/writeb.rs` |
| `READ unit var;` | Read | `statements/io/read.rs` |
| `READB unit var;` | Readb | `statements/io/readb.rs` |
| `OPENF file status access;` | Openf | `statements/io/openf.rs` |
| `OPENFB file status access;` | Openfb | `statements/io/openfb.rs` |
| `CLOSEF unit;` | Closef | `statements/io/closef.rs` |
| `OV/DAINI order nvars;` | DAInit | `statements/da/da_init.rs` |
| `DAPRV ...;` | DaPrv | `statements/da/daprv.rs` |
| `DAREV ...;` | DaRev | `statements/da/darev.rs` |
| `FIT vars; ... ENDFIT eps max algo objs;` | Fit | `statements/math/fit.rs` |

All statement AST files are under `rosy/src/program/statements/`.

## Not Yet Implemented

| COSY Construct | Type | Priority | Notes |
|----------------|------|----------|-------|
| COS | intrinsic | High | Follow SinExpr pattern |
| ABS | intrinsic | High | Follow SqrExpr pattern |
| LOG/LN | intrinsic | High | Follow ExpExpr pattern |
| SQRT | intrinsic | Medium | May be alias for SQR |
| ASIN | intrinsic | Medium | Follow SinExpr pattern |
| ATAN | intrinsic | Medium | Follow TanExpr pattern |
| RE() | conversion | Medium | Follow CM()/ST()/LO() pattern |
| ISRT | intrinsic | Medium | Inverse square root |
| NORMA | intrinsic | Low | Norm function |
| ER | intrinsic | Low | Error function |
| POLVAL | procedure | Medium | Polynomial evaluation |
| MAP | procedure | Low | Map operation |
| VELSET/VELGET | procedure | Low | Vector element access |
