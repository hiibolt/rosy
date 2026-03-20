//! # Operators
//!
//! All binary and unary operators in the ROSY language.
//!
//! - **[`arithmetic`]** — `+`, `-`, `*`, `/`
//! - **[`comparison`]** — `=`, `<>`, `<`, `>`, `<=`, `>=`
//! - **[`unary`]** — `-x` (negation), `NOT x`
//! - **[`collection`]** — `&` (concatenate), `|` (extract), `%` (DA derivative)
//!
//! The `^` (power) operator is in [`super::functions::math::exponential::pow`].
//!
//! Each operator uses a registry-driven type system defined in
//! [`rosy_lib::operators`](crate::rosy_lib::operators) that serves as the
//! single source of truth for which type combinations are valid.

pub mod arithmetic;
pub mod comparison;
pub mod unary;
pub mod collection;
