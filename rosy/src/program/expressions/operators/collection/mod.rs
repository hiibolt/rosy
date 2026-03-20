//! # Collection & DA Operators
//!
//! | Operator | Symbol | Description |
//! |----------|--------|-------------|
//! | `&` | [`concat`] | Vector/string concatenation |
//! | `\|` | [`extract`] | Element extraction (indexing) |
//! | `%` | [`derive`] | DA partial derivative |

pub mod concat;
pub mod extract;
pub mod derive;
