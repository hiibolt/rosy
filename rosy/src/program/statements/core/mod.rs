//! # Core Statements
//!
//! Fundamental control flow and declaration statements.
//!
//! | Module | Statement | Syntax |
//! |--------|-----------|--------|
//! | [`var_decl`] | Variable declaration | `VARIABLE (type) name;` |
//! | [`assign`] | Assignment | `name := expr;` |
//! | `loop` | Counted loop | `LOOP i start end [step]; ... ENDLOOP;` |
//! | [`while_loop`] | While loop | `WHILE cond; ... ENDWHILE;` |
//! | [`ploop`] | Parallel loop (MPI) | `PLOOP ... ENDPLOOP;` |
//! | `if` | Conditional | `IF cond; ... [ELSEIF ...;] [ELSE;] ENDIF;` |
//! | [`function`] | Function definition | `FUNCTION type name args; ... ENDFUNCTION;` |
//! | [`function_call`] | Function call (stmt) | `name args;` |
//! | [`procedure`] | Procedure definition | `PROCEDURE name args; ... ENDPROCEDURE;` |
//! | [`procedure_call`] | Procedure call | `name args;` |
//! | [`break_statement`] | Loop exit | `BREAK;` |

pub mod assign;
pub mod break_statement;
pub mod function_call;
pub mod function;
pub mod r#if;
pub mod r#loop;
pub mod ploop;
pub mod procedure_call;
pub mod procedure;
pub mod var_decl;
pub mod while_loop;