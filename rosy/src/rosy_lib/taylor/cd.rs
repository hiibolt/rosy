//! CD (Complex Differential Algebra) - Taylor series with complex coefficients.

use std::collections::HashMap;
use std::ops::{Add, Neg, Sub, Mul, Div};
use std::fmt;
use anyhow::Result;
use num_complex::Complex64;

use super::{Monomial, get_config, MAX_VARS};

/// Taylor polynomial with complex (Complex64) coefficients.
///
/// Represents truncated multivariate Taylor series with complex arithmetic.
/// Essential for beam physics simulations involving complex-valued functions.
#[derive(Clone, PartialEq)]
pub struct CD {
    /// Sparse coefficient storage: monomial -> complex coefficient
    coeffs: HashMap<Monomial, Complex64>,
}

impl CD {
    /// Create a new zero CD object.
    pub fn zero() -> Self {
        Self {
            coeffs: HashMap::new(),
        }
    }

    /// Create a CD constant from a real value.
    pub fn constant(value: f64) -> Self {
        Self::complex_constant(Complex64::new(value, 0.0))
    }

    /// Create a CD constant from a complex value.
    pub fn complex_constant(value: Complex64) -> Self {
        let mut coeffs = HashMap::new();
        let epsilon = get_config().ok().map(|c| c.epsilon).unwrap_or(1e-15);
        if value.norm() > epsilon {
            coeffs.insert(Monomial::constant(), value);
        }
        Self { coeffs }
    }

    /// Create a CD variable.
    ///
    /// # Arguments
    /// * `var_index` - Index of the variable (1-based, matching COSY convention)
    ///
    /// # Returns
    /// A CD representing the variable, or error if var_index is invalid
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
        coeffs.insert(Monomial::variable(var_index - 1), Complex64::new(1.0, 0.0));
        Ok(Self { coeffs })
    }

    /// Get the constant (order-0) coefficient.
    pub fn constant_part(&self) -> Complex64 {
        self.coeffs.get(&Monomial::constant()).copied().unwrap_or(Complex64::new(0.0, 0.0))
    }

    /// Get a coefficient for a specific monomial.
    pub fn get_coeff(&self, monomial: &Monomial) -> Complex64 {
        self.coeffs.get(monomial).copied().unwrap_or(Complex64::new(0.0, 0.0))
    }

    /// Set a coefficient for a specific monomial.
    pub fn set_coeff(&mut self, monomial: Monomial, value: Complex64) {
        let epsilon = get_config().ok().map(|c| c.epsilon).unwrap_or(1e-15);
        
        if value.norm() > epsilon {
            self.coeffs.insert(monomial, value);
        } else {
            self.coeffs.remove(&monomial);
        }
    }

    /// Trim small coefficients below epsilon threshold.
    pub fn trim(&mut self) {
        let epsilon = get_config().ok().map(|c| c.epsilon).unwrap_or(1e-15);
        self.coeffs.retain(|_, coeff| coeff.norm() > epsilon);
    }

    /// Get the number of non-zero coefficients.
    pub fn num_terms(&self) -> usize {
        self.coeffs.len()
    }

    /// Check if this CD is effectively zero.
    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }

    /// Get the real part as a DA.
    pub fn real_part(&self) -> super::DA {
        let real_coeffs: HashMap<Monomial, f64> = self.coeffs
            .iter()
            .map(|(&m, &c)| (m, c.re))
            .collect();
        
        super::DA::from_coeffs(real_coeffs)
    }

    /// Get the imaginary part as a DA.
    pub fn imag_part(&self) -> super::DA {
        let imag_coeffs: HashMap<Monomial, f64> = self.coeffs
            .iter()
            .map(|(&m, &c)| (m, c.im))
            .collect();
        
        super::DA::from_coeffs(imag_coeffs)
    }

    /// Create a CD from separate real and imaginary DA parts.
    ///
    /// # Arguments
    /// * `real` - The real part as a DA
    /// * `imag` - The imaginary part as a DA
    ///
    /// # Returns
    /// A CD combining the real and imaginary parts
    pub fn from_da_parts(real: &super::DA, imag: &super::DA) -> Self {
        use super::DA;
        
        // Get all monomials from both real and imaginary parts
        let mut coeffs = HashMap::new();
        
        // Add real part coefficients
        for (monomial, re_coeff) in real.coeffs_iter() {
            coeffs.insert(*monomial, Complex64::new(*re_coeff, 0.0));
        }
        
        // Add/update imaginary part coefficients
        for (monomial, im_coeff) in imag.coeffs_iter() {
            coeffs.entry(*monomial)
                .and_modify(|c| c.im = *im_coeff)
                .or_insert(Complex64::new(0.0, *im_coeff));
        }
        
        Self { coeffs }
    }

    /// Create a CD from a DA (which becomes the real part, imaginary is zero).
    ///
    /// # Arguments
    /// * `da` - The DA to use as the real part
    ///
    /// # Returns
    /// A CD with the given real part and zero imaginary part
    pub fn from_da(da: &super::DA) -> Self {
        let coeffs: HashMap<Monomial, Complex64> = da.coeffs_iter()
            .map(|(m, &v)| (*m, Complex64::new(v, 0.0)))
            .collect();
        
        Self { coeffs }
    }
}

// ============================================================================
// Arithmetic Operations
// ============================================================================

/// Addition: CD + CD
impl Add<CD> for CD {
    type Output = Result<CD>;

    fn add(self, rhs: CD) -> Self::Output {
        &self + &rhs
    }
}

/// Addition: &CD + &CD (preferred)
impl Add<&CD> for &CD {
    type Output = Result<CD>;

    fn add(self, rhs: &CD) -> Self::Output {
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
                *result.entry(*monomial).or_insert(Complex64::new(0.0, 0.0)) += coeff;
            }
        }

        // Remove small coefficients
        result.retain(|_, coeff| coeff.norm() > config.epsilon);

        Ok(CD { coeffs: result })
    }
}

/// Addition: CD + &CD
impl Add<&CD> for CD {
    type Output = Result<CD>;

    fn add(self, rhs: &CD) -> Self::Output {
        &self + rhs
    }
}

/// Addition: &CD + CD
impl Add<CD> for &CD {
    type Output = Result<CD>;

    fn add(self, rhs: CD) -> Self::Output {
        self + &rhs
    }
}

/// Addition: CD + Complex64
impl Add<Complex64> for CD {
    type Output = Result<CD>;

    fn add(self, rhs: Complex64) -> Self::Output {
        &self + rhs
    }
}

/// Addition: &CD + Complex64
impl Add<Complex64> for &CD {
    type Output = Result<CD>;

    fn add(self, rhs: Complex64) -> Self::Output {
        let config = get_config()?;
        let mut result = self.clone();
        
        let const_monomial = Monomial::constant();
        *result.coeffs.entry(const_monomial).or_insert(Complex64::new(0.0, 0.0)) += rhs;
        
        // Trim if needed
        if let Some(&coeff) = result.coeffs.get(&const_monomial) {
            if coeff.norm() <= config.epsilon {
                result.coeffs.remove(&const_monomial);
            }
        }
        
        Ok(result)
    }
}

/// Addition: Complex64 + CD
impl Add<CD> for Complex64 {
    type Output = Result<CD>;

    fn add(self, rhs: CD) -> Self::Output {
        &rhs + self
    }
}

/// Addition: Complex64 + &CD
impl Add<&CD> for Complex64 {
    type Output = Result<CD>;

    fn add(self, rhs: &CD) -> Self::Output {
        rhs + self
    }
}

/// Negation: -CD
impl Neg for CD {
    type Output = CD;

    fn neg(self) -> Self::Output {
        -&self
    }
}

/// Negation: -&CD
impl Neg for &CD {
    type Output = CD;

    fn neg(self) -> Self::Output {
        CD {
            coeffs: self.coeffs.iter().map(|(&m, &c)| (m, -c)).collect(),
        }
    }
}

/// Subtraction: CD - CD
impl Sub<CD> for CD {
    type Output = Result<CD>;

    fn sub(self, rhs: CD) -> Self::Output {
        &self - &rhs
    }
}

/// Subtraction: &CD - &CD
impl Sub<&CD> for &CD {
    type Output = Result<CD>;

    fn sub(self, rhs: &CD) -> Self::Output {
        self + &(-rhs)
    }
}

/// Subtraction: CD - &CD
impl Sub<&CD> for CD {
    type Output = Result<CD>;

    fn sub(self, rhs: &CD) -> Self::Output {
        &self - rhs
    }
}

/// Subtraction: &CD - CD
impl Sub<CD> for &CD {
    type Output = Result<CD>;

    fn sub(self, rhs: CD) -> Self::Output {
        self - &rhs
    }
}

/// Subtraction: CD - Complex64
impl Sub<Complex64> for CD {
    type Output = Result<CD>;

    fn sub(self, rhs: Complex64) -> Self::Output {
        &self - rhs
    }
}

/// Subtraction: &CD - Complex64
impl Sub<Complex64> for &CD {
    type Output = Result<CD>;

    fn sub(self, rhs: Complex64) -> Self::Output {
        self + (-rhs)
    }
}

/// Subtraction: Complex64 - CD
impl Sub<CD> for Complex64 {
    type Output = Result<CD>;

    fn sub(self, rhs: CD) -> Self::Output {
        self - &rhs
    }
}

/// Subtraction: Complex64 - &CD
impl Sub<&CD> for Complex64 {
    type Output = Result<CD>;

    fn sub(self, rhs: &CD) -> Self::Output {
        self + &(-rhs)
    }
}

// ============================================================================
// Multiplication Operations
// ============================================================================

/// Multiplication: CD * CD
impl Mul<CD> for CD {
    type Output = Result<CD>;

    fn mul(self, rhs: CD) -> Self::Output {
        &self * &rhs
    }
}

/// Multiplication: &CD * &CD (preferred - implements actual CD multiplication)
impl Mul<&CD> for &CD {
    type Output = Result<CD>;

    fn mul(self, rhs: &CD) -> Self::Output {
        let config = get_config()?;
        let mut result = HashMap::new();

        // Multiply each term in self by each term in rhs
        for (m1, &c1) in &self.coeffs {
            for (m2, &c2) in &rhs.coeffs {
                let product_monomial = m1.multiply(m2);
                
                // Only include terms within the truncation order
                if product_monomial.within_order(config.max_order) {
                    *result.entry(product_monomial).or_insert(Complex64::new(0.0, 0.0)) += c1 * c2;
                }
            }
        }

        // Remove small coefficients
        result.retain(|_, coeff| coeff.norm() > config.epsilon);

        Ok(CD { coeffs: result })
    }
}

/// Multiplication: CD * &CD
impl Mul<&CD> for CD {
    type Output = Result<CD>;

    fn mul(self, rhs: &CD) -> Self::Output {
        &self * rhs
    }
}

/// Multiplication: &CD * CD
impl Mul<CD> for &CD {
    type Output = Result<CD>;

    fn mul(self, rhs: CD) -> Self::Output {
        self * &rhs
    }
}

/// Multiplication: CD * Complex64
impl Mul<Complex64> for CD {
    type Output = Result<CD>;

    fn mul(self, rhs: Complex64) -> Self::Output {
        &self * rhs
    }
}

/// Multiplication: &CD * Complex64 (scalar multiplication)
impl Mul<Complex64> for &CD {
    type Output = Result<CD>;

    fn mul(self, rhs: Complex64) -> Self::Output {
        let config = get_config()?;
        
        if rhs.norm() <= config.epsilon {
            return Ok(CD::zero());
        }

        let coeffs: HashMap<Monomial, Complex64> = self.coeffs
            .iter()
            .map(|(&m, &c)| (m, c * rhs))
            .collect();

        Ok(CD { coeffs })
    }
}

/// Multiplication: Complex64 * CD
impl Mul<CD> for Complex64 {
    type Output = Result<CD>;

    fn mul(self, rhs: CD) -> Self::Output {
        &rhs * self
    }
}

/// Multiplication: Complex64 * &CD
impl Mul<&CD> for Complex64 {
    type Output = Result<CD>;

    fn mul(self, rhs: &CD) -> Self::Output {
        rhs * self
    }
}

// ============================================================================
// Division
// ============================================================================

/// Division: CD / CD
impl Div<CD> for CD {
    type Output = Result<CD>;

    fn div(self, rhs: CD) -> Self::Output {
        &self / &rhs
    }
}

/// Division: &CD / &CD (preferred - avoids clones)
impl Div<&CD> for &CD {
    type Output = Result<CD>;

    fn div(self, rhs: &CD) -> Self::Output {
        // For complex Taylor series division, we use: f/g = f * (1/g)
        let g0 = rhs.constant_part();
        
        if g0.norm() < 1e-15 {
            anyhow::bail!("Division by zero: divisor has zero constant term");
        }
        
        // Simple implementation: compute 1/g via Newton iteration
        // h = 1/g, then h_{n+1} = h_n * (2 - g * h_n)
        let mut h = CD::complex_constant(Complex64::new(1.0, 0.0) / g0);
        
        // Perform Newton iterations (20 iterations for high precision)
        for _ in 0..20 {
            let gh = (rhs * &h)?;
            let two_minus_gh = (&CD::constant(2.0) - &gh)?;
            let new_h = (&h * &two_minus_gh)?;
            h = new_h;
        }
        
        // Multiply self by the reciprocal
        self * &h
    }
}

/// Division: &CD / CD
impl Div<CD> for &CD {
    type Output = Result<CD>;

    fn div(self, rhs: CD) -> Self::Output {
        self / &rhs
    }
}

/// Division: CD / &CD
impl Div<&CD> for CD {
    type Output = Result<CD>;

    fn div(self, rhs: &CD) -> Self::Output {
        &self / rhs
    }
}

/// Division: CD / Complex64
impl Div<Complex64> for CD {
    type Output = Result<CD>;

    fn div(self, rhs: Complex64) -> Self::Output {
        &self / rhs
    }
}

/// Division: &CD / Complex64 (preferred - avoids clones)
impl Div<Complex64> for &CD {
    type Output = Result<CD>;

    fn div(self, rhs: Complex64) -> Self::Output {
        if rhs.norm() < 1e-15 {
            anyhow::bail!("Division by zero");
        }
        
        let coeffs: HashMap<Monomial, Complex64> = self.coeffs
            .iter()
            .map(|(&m, &c)| (m, c / rhs))
            .collect();

        Ok(CD { coeffs })
    }
}

/// Division: Complex64 / CD
impl Div<CD> for Complex64 {
    type Output = Result<CD>;

    fn div(self, rhs: CD) -> Self::Output {
        &CD::complex_constant(self) / &rhs
    }
}

/// Division: Complex64 / &CD
impl Div<&CD> for Complex64 {
    type Output = Result<CD>;

    fn div(self, rhs: &CD) -> Self::Output {
        &CD::complex_constant(self) / rhs
    }
}

// ============================================================================
// Display & Debug
// ============================================================================

impl fmt::Debug for CD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CD[")?;
        let mut sorted: Vec<_> = self.coeffs.iter().collect();
        sorted.sort_by_key(|(m, _)| *m);
        
        for (i, (monomial, coeff)) in sorted.iter().enumerate() {
            if i > 0 {
                write!(f, " + ")?;
            }
            write!(f, "({:.6}{:+.6}i)*{}", coeff.re, coeff.im, monomial)?;
        }
        if sorted.is_empty() {
            write!(f, "0")?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for CD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.coeffs.is_empty() {
            return write!(f, "0");
        }

        let mut sorted: Vec<_> = self.coeffs.iter().collect();
        sorted.sort_by_key(|(m, _)| *m);
        
        for (i, (monomial, coeff)) in sorted.iter().enumerate() {
            if i > 0 {
                write!(f, " + ")?;
            }
            write!(f, "({:.6}{:+.6}i)", coeff.re, coeff.im)?;
            if monomial.total_order > 0 {
                write!(f, "*{}", monomial)?;
            }
        }
        Ok(())
    }
}