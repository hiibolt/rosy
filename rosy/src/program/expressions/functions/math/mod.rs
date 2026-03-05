//! # Mathematical Functions
//!
//! Built-in math functions and operators.
//!
//! | Module | Function | Description |
//! |--------|----------|-------------|
//! | [`trig`] | `SIN`, `TAN` | Trigonometric functions |
//! | [`exp`] | `EXP` | Exponential (e^x) |
//! | [`sqr`] | `SQR` | Square root |
//! | [`pow`] | `^` | Power / exponentiation |
//! | [`vmax`] | `VMAX` | Vector maximum |
//! | [`lst`] | `LST` | String memory estimate (COSY compat) |
//! | [`lcm`] | `LCM` | Complex memory estimate (COSY compat) |
//! | [`lcd`] | `LCD` | DA memory estimate (COSY compat) |

pub mod trig;
pub mod exp;
pub mod lcd;
pub mod lcm;
pub mod lst;
pub mod pow;
pub mod sqr;
pub mod vmax;