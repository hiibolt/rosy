//! # Core Expression Nodes
//!
//! Fundamental expression building blocks shared across the AST.
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`var_expr`] | Variable references & function call disambiguation |
//! | [`variable_identifier`] | Parsed identifier with optional indexing / arguments |

pub mod var_expr;
pub mod variable_identifier;