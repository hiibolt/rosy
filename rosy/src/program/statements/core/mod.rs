//! # Core Statements
//!
//! Declarations, control flow, functions, and procedures.
//!
//! ## Declarations & Assignment
//!
//! - **[`var_decl`]** — `VARIABLE (type) name;`
//! - **[`assign`]** — `name := expr;`
//!
//! ## Control Flow
//!
//! - **[`r#loop`]** — `LOOP i start end [step]; ... ENDLOOP;`
//! - **[`while_loop`]** — `WHILE cond; ... ENDWHILE;`
//! - **[`ploop`]** — `PLOOP ... ENDPLOOP;` (MPI parallel)
//! - **[`r#if`]** — `IF cond; ... [ELSEIF ...;] [ELSE;] ENDIF;`
//! - **[`break_statement`]** — `BREAK;`
//! - **[`quit`]** — `QUIT;`
//!
//! ## Functions & Procedures
//!
//! - **[`function`]** / **[`function_call`]** — define and call functions
//! - **[`procedure`]** / **[`procedure_call`]** — define and call procedures

pub mod assign;
pub mod break_statement;
pub mod function_call;
pub mod function;
pub mod r#if;
pub mod r#loop;
pub mod ploop;
pub mod procedure_call;
pub mod procedure;
pub mod quit;
pub mod scrlen;
pub mod substr;
pub mod var_decl;
pub mod velset;
pub mod while_loop;