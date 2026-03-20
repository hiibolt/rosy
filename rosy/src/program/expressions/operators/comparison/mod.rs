//! # Comparison Operators
//!
//! All comparisons return `LO` (logical/boolean).
//!
//! | Operator | Symbol | Description |
//! |----------|--------|-------------|
//! | `=` | [`eq`] | Equal |
//! | `<>` | [`neq`] | Not equal |
//! | `<` | [`lt`] | Less than |
//! | `>` | [`gt`] | Greater than |
//! | `<=` | [`lte`] | Less than or equal |
//! | `>=` | [`gte`] | Greater than or equal |

pub mod eq;
pub mod neq;
pub mod lt;
pub mod gt;
pub mod lte;
pub mod gte;
