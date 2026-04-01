//! # Math Statements
//!
//! Mathematical and linear algebra operations.
//!
//! - **[`fit`]** — `FIT ... ENDFIT;` — optimization loop
//! - **[`ldet`]** — `LDET mat var;` — matrix determinant
//! - **[`lev`]** — `LEV mat er ei ev n d;` — eigenvalues and eigenvectors
//! - **[`linv`]** — `LINV mat inv;` — matrix inverse
//! - **[`mblock`]** — `MBLOCK mat T Ti d n;` — block-diagonal transform
//! - **[`polval`]** — `POLVAL coeffs x result;` — polynomial evaluation
//! - **[`vedot`]** — `VEDOT v1 v2 result;` — vector dot product
//! - **[`veunit`]** — `VEUNIT vec result;` — normalize to unit vector
//! - **[`vezero`]** — `VEZERO arr n thresh;` — zero components past threshold

pub mod fit;
pub mod ldet;
pub mod lev;
pub mod linv;
pub mod lsline;
pub mod mblock;
pub mod polval;
pub mod rkco;
pub mod vedot;
pub mod veunit;
pub mod vezero;