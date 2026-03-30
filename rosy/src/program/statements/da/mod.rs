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
//! - **[`mtree`]** — `MTREE ...;` — tree representation for fast DA evaluation

pub mod da_init;
pub mod dascl;
pub mod dasgn;
pub mod dader;
pub mod daint;
pub mod danoro;
pub mod danors;
pub mod daeps;
pub mod danot;
pub mod daprv;
pub mod darev;
pub mod datrn;
pub mod mtree;
pub mod dacliw;
pub mod dacqlc;
pub mod dadiu;
pub mod dadmu;
pub mod daplu;
pub mod darea;
pub mod dapew;
pub mod dapee;
pub mod dapea;
pub mod dapep;
pub mod daest;