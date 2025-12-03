//! Taylor series implementation for differential algebra.
//!
//! This module provides DA (real) and CD (complex) differential algebra types
//! for automatic differentiation and polynomial manipulation in beam physics simulations.

mod monomial;
mod config;
mod da;

pub use monomial::Monomial;
pub use config::{TaylorConfig, init_taylor, get_config};
pub use da::DACoefficient;

// Core generic differential algebra type
use num_complex::Complex64;

/// Real differential algebra (f64 coefficients) - traditional DA
pub type DA = da::DA<f64>;

/// Complex differential algebra (Complex64 coefficients) - replaces CD
pub type CD = da::DA<Complex64>;

/// Maximum number of variables supported.
/// 
/// Set to 16 to handle typical beam physics cases:
/// - 6D phase space (x, px, y, py, z, pz)
/// - Additional coupling/parameter variables
pub const MAX_VARS: usize = 6;

/// Default epsilon for coefficient truncation.
pub const DEFAULT_EPSILON: f64 = 1e-15;
