//! # Built-in Functions
//!
//! Intrinsic functions provided by the ROSY language.
//!
//! ## Sub-modules
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`conversion`] | Type conversion — `CM()`, `ST()`, `LO()` |
//! | [`math`] | Mathematical — `SIN`, `TAN`, `SQR`, `EXP`, `^`, `VMAX`, memory estimators |
//! | [`sys`] | System utilities — `LENGTH` |

pub mod conversion;
pub mod math;
pub mod sys;