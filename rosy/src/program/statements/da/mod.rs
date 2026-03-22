//! # Differential Algebra (DA) Statements
//!
//! Statements for initializing and working with Taylor series (DA/CD) values.
//!
//! - **[`da_init`]** — `OV order nvars;` — initialize the DA environment
//! - **[`daprv`]** — `DAPRV ...;` — print DA values
//! - **[`darev`]** — `DAREV ...;` — reverse-print DA values
//! - **[`daeps`]** — `DAEPS eps;` — set DA epsilon
//! - **[`danot`]** — `DANOT order;` — set DA notation order
//! - **[`datrn`]** — `DATRN var;` — DA truncation

pub mod da_init;
pub mod daeps;
pub mod danot;
pub mod daprv;
pub mod darev;
pub mod datrn;
pub mod mtree;