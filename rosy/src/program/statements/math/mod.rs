//! # Math Statements
//!
//! Mathematical and linear algebra operations.
//!
//! - **[`fit`]** — `FIT ... ENDFIT;` — optimization loop
//! - **[`ldet`]** — `LDET mat var;` — matrix determinant
//! - **[`linv`]** — `LINV mat inv;` — matrix inverse
//! - **[`polval`]** — `POLVAL coeffs x result;` — polynomial evaluation

pub mod fit;
pub mod ldet;
pub mod linv;
pub mod polval;
pub mod vedot;
pub mod veunit;
pub mod vezero;