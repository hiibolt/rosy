//! DA (Differential Algebra) - Taylor series with real coefficients.

use std::collections::HashMap;
use std::ops::{Add, Neg, Sub};
use std::fmt;
use anyhow::{Result, Context};

use super::{Monomial, get_config, MAX_VARS};

/// Taylor polynomial with real (f64) coefficients.
///
/// Represents truncated multivariate Taylor series for automatic differentiation.
/// Optimized for sparse representation - most polynomials have <100 non-zero terms.
#[derive(Clone, PartialEq)]
pub struct DA {
    /// Sparse coefficient storage: monomial -> coefficient
    coeffs: HashMap<Monomial, f64>,
}

impl DA {
    /// Create a new zero DA object.
    pub fn zero() -> Self {
        Self {
            coeffs: HashMap::new(),
        }
    }

    /// Create a DA constant.
    ///
    /// # Arguments
    /// * `value` - The constant value
    pub fn constant(value: f64) -> Self {
        let mut coeffs = HashMap::new();
        if value.abs() > get_config().ok().map(|c| c.epsilon).unwrap_or(1e-15) {
            coeffs.insert(Monomial::constant(), value);
        }
        Self { coeffs }
    }

    /// Create a DA variable.
    ///
    /// # Arguments
    /// * `var_index` - Index of the variable (1-based, matching COSY convention)
    ///
    /// # Returns
    /// A DA representing the variable, or error if var_index is invalid
    ///
    /// # Example
    /// ```
    /// let x1 = DA::variable(1)?;  // First variable
    /// let x2 = DA::variable(2)?;  // Second variable
    /// ```
    pub fn variable(var_index: usize) -> Result<Self> {
        let config = get_config()?;
        
        if var_index == 0 || var_index > config.num_vars {
            anyhow::bail!(
                "Variable index {} out of range [1, {}]",
                var_index,
                config.num_vars
            );
        }

        let mut coeffs = HashMap::new();
        coeffs.insert(Monomial::variable(var_index - 1), 1.0);
        Ok(Self { coeffs })
    }

    /// Get the constant (order-0) coefficient.
    pub fn constant_part(&self) -> f64 {
        self.coeffs.get(&Monomial::constant()).copied().unwrap_or(0.0)
    }

    /// Get a coefficient for a specific monomial.
    pub fn get_coeff(&self, monomial: &Monomial) -> f64 {
        self.coeffs.get(monomial).copied().unwrap_or(0.0)
    }

    /// Set a coefficient for a specific monomial.
    ///
    /// Small coefficients (< epsilon) are automatically removed.
    pub fn set_coeff(&mut self, monomial: Monomial, value: f64) {
        let epsilon = get_config().ok().map(|c| c.epsilon).unwrap_or(1e-15);
        
        if value.abs() > epsilon {
            self.coeffs.insert(monomial, value);
        } else {
            self.coeffs.remove(&monomial);
        }
    }

    /// Trim small coefficients below epsilon threshold.
    pub fn trim(&mut self) {
        let epsilon = get_config().ok().map(|c| c.epsilon).unwrap_or(1e-15);
        self.coeffs.retain(|_, &mut coeff| coeff.abs() > epsilon);
    }

    /// Get the number of non-zero coefficients.
    pub fn num_terms(&self) -> usize {
        self.coeffs.len()
    }

    /// Check if this DA is effectively zero.
    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }

    /// Create a DA from raw coefficients (used internally by CD).
    pub(crate) fn from_coeffs(coeffs: HashMap<Monomial, f64>) -> Self {
        Self { coeffs }
    }

    /// Iterate over coefficients (used by CD for conversion).
    pub(crate) fn coeffs_iter(&self) -> impl Iterator<Item = (&Monomial, &f64)> {
        self.coeffs.iter()
    }
}

// ============================================================================
// Arithmetic Operations
// ============================================================================

/// Addition: DA + DA
impl Add<DA> for DA {
    type Output = Result<DA>;

    fn add(self, rhs: DA) -> Self::Output {
        &self + &rhs
    }
}

/// Addition: &DA + &DA (preferred - avoids clones)
impl Add<&DA> for &DA {
    type Output = Result<DA>;

    fn add(self, rhs: &DA) -> Self::Output {
        let config = get_config()?;
        let mut result = HashMap::new();

        // Add coefficients from self
        for (monomial, &coeff) in &self.coeffs {
            if monomial.within_order(config.max_order) {
                result.insert(*monomial, coeff);
            }
        }

        // Add coefficients from rhs
        for (monomial, &coeff) in &rhs.coeffs {
            if monomial.within_order(config.max_order) {
                *result.entry(*monomial).or_insert(0.0) += coeff;
            }
        }

        // Remove small coefficients
        result.retain(|_, &mut coeff| coeff.abs() > config.epsilon);

        Ok(DA { coeffs: result })
    }
}

/// Addition: DA + &DA
impl Add<&DA> for DA {
    type Output = Result<DA>;

    fn add(self, rhs: &DA) -> Self::Output {
        &self + rhs
    }
}

/// Addition: &DA + DA
impl Add<DA> for &DA {
    type Output = Result<DA>;

    fn add(self, rhs: DA) -> Self::Output {
        self + &rhs
    }
}

/// Addition: DA + f64
impl Add<f64> for DA {
    type Output = Result<DA>;

    fn add(self, rhs: f64) -> Self::Output {
        &self + rhs
    }
}

/// Addition: &DA + f64
impl Add<f64> for &DA {
    type Output = Result<DA>;

    fn add(self, rhs: f64) -> Self::Output {
        let config = get_config()?;
        let mut result = self.clone();
        
        let const_monomial = Monomial::constant();
        *result.coeffs.entry(const_monomial).or_insert(0.0) += rhs;
        
        // Trim if needed
        if let Some(&coeff) = result.coeffs.get(&const_monomial) {
            if coeff.abs() <= config.epsilon {
                result.coeffs.remove(&const_monomial);
            }
        }
        
        Ok(result)
    }
}

/// Addition: f64 + DA
impl Add<DA> for f64 {
    type Output = Result<DA>;

    fn add(self, rhs: DA) -> Self::Output {
        &rhs + self
    }
}

/// Addition: f64 + &DA
impl Add<&DA> for f64 {
    type Output = Result<DA>;

    fn add(self, rhs: &DA) -> Self::Output {
        rhs + self
    }
}

/// Negation: -DA
impl Neg for DA {
    type Output = DA;

    fn neg(self) -> Self::Output {
        -&self
    }
}

/// Negation: -&DA
impl Neg for &DA {
    type Output = DA;

    fn neg(self) -> Self::Output {
        DA {
            coeffs: self.coeffs.iter().map(|(&m, &c)| (m, -c)).collect(),
        }
    }
}

/// Subtraction: DA - DA
impl Sub<DA> for DA {
    type Output = Result<DA>;

    fn sub(self, rhs: DA) -> Self::Output {
        &self - &rhs
    }
}

/// Subtraction: &DA - &DA
impl Sub<&DA> for &DA {
    type Output = Result<DA>;

    fn sub(self, rhs: &DA) -> Self::Output {
        self + &(-rhs)
    }
}

/// Subtraction: DA - &DA
impl Sub<&DA> for DA {
    type Output = Result<DA>;

    fn sub(self, rhs: &DA) -> Self::Output {
        &self - rhs
    }
}

/// Subtraction: &DA - DA
impl Sub<DA> for &DA {
    type Output = Result<DA>;

    fn sub(self, rhs: DA) -> Self::Output {
        self - &rhs
    }
}

/// Subtraction: DA - f64
impl Sub<f64> for DA {
    type Output = Result<DA>;

    fn sub(self, rhs: f64) -> Self::Output {
        &self - rhs
    }
}

/// Subtraction: &DA - f64
impl Sub<f64> for &DA {
    type Output = Result<DA>;

    fn sub(self, rhs: f64) -> Self::Output {
        self + (-rhs)
    }
}

/// Subtraction: f64 - DA
impl Sub<DA> for f64 {
    type Output = Result<DA>;

    fn sub(self, rhs: DA) -> Self::Output {
        self - &rhs
    }
}

/// Subtraction: f64 - &DA
impl Sub<&DA> for f64 {
    type Output = Result<DA>;

    fn sub(self, rhs: &DA) -> Self::Output {
        self + &(-rhs)
    }
}

// ============================================================================
// Display & Debug
// ============================================================================

impl fmt::Debug for DA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DA[")?;
        let mut sorted: Vec<_> = self.coeffs.iter().collect();
        sorted.sort_by_key(|(m, _)| *m);
        
        for (i, (monomial, coeff)) in sorted.iter().enumerate() {
            if i > 0 {
                write!(f, " + ")?;
            }
            write!(f, "{:.6}*{}", coeff, monomial)?;
        }
        if sorted.is_empty() {
            write!(f, "0")?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for DA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.coeffs.is_empty() {
            return write!(f, "0");
        }

        let mut sorted: Vec<_> = self.coeffs.iter().collect();
        sorted.sort_by_key(|(m, _)| *m);
        
        for (i, (monomial, coeff)) in sorted.iter().enumerate() {
            if i > 0 {
                if *coeff >= &0.0 {
                    write!(f, " + ")?;
                } else {
                    write!(f, " - ")?;
                    write!(f, "{:.6}*{}", -*coeff, monomial)?;
                    continue;
                }
            }
            write!(f, "{:.6}", coeff)?;
            if monomial.total_order > 0 {
                write!(f, "*{}", monomial)?;
            }
        }
        Ok(())
    }
}