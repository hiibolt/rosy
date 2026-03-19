//! # Differential Algebra (DA) Statements
//!
//! Statements for initializing and printing Taylor series (DA/CD) values.
//!
//! | Module | Statement | Description |
//! |--------|-----------|-------------|
//! | [`da_init`] | `OV order nvars;` | Initialize the Taylor series environment |
//! | [`daprv`] | `DAPRV ...;` | Print DA array values |
//! | [`darev`] | `DAREV ...;` | Reverse-print DA array values |

pub mod da_init;
pub mod daeps;
pub mod danot;
pub mod daprv;
pub mod darev;
pub mod datrn;