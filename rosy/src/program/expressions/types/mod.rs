//! # Type Literals
//!
//! AST representations of literal values in the ROSY language.
//!
//! | Module | ROSY Type | Rust Type | Example Literal |
//! |--------|-----------|-----------|-----------------|
//! | [`number`] | `RE` | `f64` | `3.14`, `42`, `-7` |
//! | [`string`] | `ST` | `String` | `'hello'` |
//! | [`boolean`] | `LO` | `bool` | `TRUE`, `FALSE` |
//! | [`da`] | `DA` | Taylor series | `DA(1)` |
//! | [`cd`] | `CD` | Complex Taylor | `CD(1)` |

pub mod boolean;
pub mod cd;
pub mod da;
pub mod number;
pub mod string;