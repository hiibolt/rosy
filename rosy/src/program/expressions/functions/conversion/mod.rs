//! # Type Conversion Functions
//!
//! Functions that convert values between ROSY base types.
//!
//! | Function | Description |
//! |----------|-------------|
//! | [`complex_convert`] `CM(expr)` | Convert to complex number |
//! | [`string_convert`] `ST(expr)` | Convert to string |
//! | [`logical_convert`] `LO(expr)` | Convert to logical (boolean) |

pub mod complex_convert;
pub mod logical_convert;
pub mod string_convert;