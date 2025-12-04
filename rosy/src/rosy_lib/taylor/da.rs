//! DA (Differential Algebra) - Generic Taylor series implementation.

use std::collections::HashMap;
use std::f32::consts::E;
use std::ops::{Add, Neg, Sub, Mul, Div, AddAssign};
use std::fmt;
use anyhow::{Result, Context};
use num_complex::Complex64;

use super::{Monomial, get_config, MAX_VARS};

/// Trait for types that can be used as Taylor series coefficients.
///
/// Requires arithmetic operations, zero/one identities, and magnitude measurement.
pub trait DACoefficient: 
    Clone + Copy + 
    Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + 
    AddAssign + MulAdd + 
    Neg<Output=Self> +
    PartialEq + fmt::Debug + fmt::Display
{
    /// The additive identity (0).
    fn zero() -> Self;
    
    /// The multiplicative identity (1).
    fn one() -> Self;
    
    /// Magnitude/absolute value for epsilon comparisons.
    fn abs(&self) -> f64;
}

impl DACoefficient for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn abs(&self) -> f64 { f64::abs(*self) }
}

// Extension trait for FMA operations (when available)
// f64 has hardware FMA on most modern CPUs
pub trait MulAdd {
    fn mul_add(self, a: Self, b: Self) -> Self;
}

impl MulAdd for f64 {
    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        // Hardware FMA: self * a + b
        // Single rounding operation, more accurate than separate mul+add
        f64::mul_add(self, a, b)
    }
}

impl MulAdd for Complex64 {
    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        // Complex multiplication doesn't have hardware FMA, but still more efficient
        // than separate operations due to instruction pipelining
        self * a + b
    }
}

impl DACoefficient for Complex64 {
    fn zero() -> Self { Complex64::new(0.0, 0.0) }
    fn one() -> Self { Complex64::new(1.0, 0.0) }
    fn abs(&self) -> f64 { self.norm() }
}

/// Generic Taylor polynomial with type-parameterized coefficients.
///
/// Represents truncated multivariate Taylor series for automatic differentiation.
/// Optimized for sparse representation - most polynomials have <100 non-zero terms.
///
/// Type parameter `T` can be:
/// - `f64` for real differential algebra (traditional DA)
/// - `Complex64` for complex differential algebra (CD)
#[derive(Clone, PartialEq)]
pub struct DA<T: DACoefficient> {
    /// Sparse coefficient storage: monomial -> coefficient
    coeffs: HashMap<Monomial, T>,
}

impl<T: DACoefficient> DA<T> {
    /// Create a new zero DA object.
    pub fn zero() -> Self {
        Self {
            coeffs: HashMap::new(),
        }
    }

    /// Create a DA constant from a coefficient value.
    ///
    /// # Arguments
    /// * `value` - The constant coefficient value
    pub fn from_coeff(value: T) -> Self {
        let mut coeffs = HashMap::new();
        let epsilon = get_config().ok().map(|c| c.epsilon).unwrap_or(1e-15);
        if value.abs() > epsilon {
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
        coeffs.insert(Monomial::variable(var_index - 1), T::one());
        Ok(Self { coeffs })
    }

    /// Get the constant (order-0) coefficient.
    pub fn constant_part(&self) -> T {
        self.coeffs.get(&Monomial::constant()).copied().unwrap_or(T::zero())
    }

    /// Get a coefficient for a specific monomial.
    pub fn get_coeff(&self, monomial: &Monomial) -> T {
        self.coeffs.get(monomial).copied().unwrap_or(T::zero())
    }

    /// Set a coefficient for a specific monomial.
    ///
    /// Small coefficients (< epsilon) are automatically removed.
    pub fn set_coeff(&mut self, monomial: Monomial, value: T) {
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
        self.coeffs.retain(|_, coeff| coeff.abs() > epsilon);
    }

    /// Get the number of non-zero coefficients.
    pub fn num_terms(&self) -> usize {
        self.coeffs.len()
    }

    /// Check if this DA is effectively zero.
    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }

    /// Create a DA from raw coefficients (used internally).
    pub(crate) fn from_coeffs(coeffs: HashMap<Monomial, T>) -> Self {
        Self { coeffs }
    }

    /// Iterate over coefficients.
    pub fn coeffs_iter(&self) -> impl Iterator<Item = (&Monomial, &T)> {
        self.coeffs.iter()
    }
}

// ============================================================================
// Arithmetic Operations
// ============================================================================

/// Addition: DA + DA
impl<T: DACoefficient> Add<DA<T>> for DA<T> {
    type Output = Result<DA<T>>;

    fn add(self, rhs: DA<T>) -> Self::Output {
        &self + &rhs
    }
}

/// Addition: &DA + &DA (preferred - avoids clones)
impl<T: DACoefficient> Add<&DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn add(self, rhs: &DA<T>) -> Self::Output {
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
                *result.entry(*monomial).or_insert(T::zero()) += coeff;
            }
        }

        // Remove small coefficients
        result.retain(|_, &mut coeff| coeff.abs() > config.epsilon);

        Ok(DA { coeffs: result })
    }
}

/// Addition: DA + &DA
impl<T: DACoefficient> Add<&DA<T>> for DA<T> {
    type Output = Result<DA<T>>;

    fn add(self, rhs: &DA<T>) -> Self::Output {
        &self + rhs
    }
}

/// Addition: &DA + DA
impl<T: DACoefficient> Add<DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn add(self, rhs: DA<T>) -> Self::Output {
        self + &rhs
    }
}

/// Addition: DA + T
impl<T: DACoefficient> Add<T> for DA<T> {
    type Output = Result<DA<T>>;

    fn add(self, rhs: T) -> Self::Output {
        &self + rhs
    }
}

/// Addition: &DA + T
impl<T: DACoefficient> Add<T> for &DA<T> {
    type Output = Result<DA<T>>;

    fn add(self, rhs: T) -> Self::Output {
        let config = get_config()?;
        let mut result = self.clone();
        
        let const_monomial = Monomial::constant();
        *result.coeffs.entry(const_monomial).or_insert(T::zero()) += rhs;
        
        // Trim if needed
        if let Some(&coeff) = result.coeffs.get(&const_monomial) {
            if coeff.abs() <= config.epsilon {
                result.coeffs.remove(&const_monomial);
            }
        }
        
        Ok(result)
    }
}

// ============================================================================
// Note: T + DA and T + &DA impls removed due to Rust orphan rules
// Users should write DA + T instead
// ============================================================================

/// Negation: -DA
impl<T: DACoefficient> Neg for DA<T> {
    type Output = DA<T>;

    fn neg(self) -> Self::Output {
        -&self
    }
}

/// Negation: -&DA
impl<T: DACoefficient> Neg for &DA<T> {
    type Output = DA<T>;

    fn neg(self) -> Self::Output {
        DA {
            coeffs: self.coeffs.iter().map(|(&m, &c)| (m, -c)).collect(),
        }
    }
}

/// Subtraction: DA - DA
impl<T: DACoefficient> Sub<DA<T>> for DA<T> {
    type Output = Result<DA<T>>;

    fn sub(self, rhs: DA<T>) -> Self::Output {
        &self - &rhs
    }
}

/// Subtraction: &DA - &DA
impl<T: DACoefficient> Sub<&DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn sub(self, rhs: &DA<T>) -> Self::Output {
        self + &(-rhs)
    }
}

/// Subtraction: DA - &DA
impl<T: DACoefficient> Sub<&DA<T>> for DA<T> {
    type Output = Result<DA<T>>;

    fn sub(self, rhs: &DA<T>) -> Self::Output {
        &self - rhs
    }
}

/// Subtraction: &DA - DA
impl<T: DACoefficient> Sub<DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn sub(self, rhs: DA<T>) -> Self::Output {
        self - &rhs
    }
}

/// Subtraction: DA - T
impl<T: DACoefficient> Sub<T> for DA<T> {
    type Output = Result<DA<T>>;

    fn sub(self, rhs: T) -> Self::Output {
        &self - rhs
    }
}

/// Subtraction: &DA - T
impl<T: DACoefficient> Sub<T> for &DA<T> {
    type Output = Result<DA<T>>;

    fn sub(self, rhs: T) -> Self::Output {
        self + (-rhs)
    }
}

// ============================================================================
// Note: T - DA and T - &DA impls removed due to Rust orphan rules
// Users should write -(DA - T) or similar patterns
// ============================================================================

// ============================================================================
// Multiplication Operations
// ============================================================================

/// Multiplication: DA * DA
impl<T: DACoefficient> Mul<DA<T>> for DA<T> {
    type Output = Result<DA<T>>;

    fn mul(self, rhs: DA<T>) -> Self::Output {
        &self * &rhs
    }
}

/// Multiplication: &DA * &DA (preferred - implements actual DA multiplication)
impl<T: DACoefficient> Mul<&DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn mul(self, rhs: &DA<T>) -> Self::Output {
        let config = get_config()?;
        let mut result = HashMap::new();

        // Multiply each term in self by each term in rhs
        for (m1, &c1) in &self.coeffs {
            for (m2, &c2) in &rhs.coeffs {
                let product_monomial = m1.multiply(m2);
                
                // Only include terms within the truncation order
                if product_monomial.within_order(config.max_order) {
                    *result.entry(product_monomial).or_insert(T::zero()) += c1 * c2;
                }
            }
        }

        // Remove small coefficients
        result.retain(|_, &mut coeff| coeff.abs() > config.epsilon);

        Ok(DA { coeffs: result })
    }
}

/// Multiplication: DA * &DA
impl<T: DACoefficient> Mul<&DA<T>> for DA<T> {
    type Output = Result<DA<T>>;

    fn mul(self, rhs: &DA<T>) -> Self::Output {
        &self * rhs
    }
}

/// Multiplication: &DA * DA
impl<T: DACoefficient> Mul<DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn mul(self, rhs: DA<T>) -> Self::Output {
        self * &rhs
    }
}

/// Multiplication: DA * T
impl<T: DACoefficient> Mul<T> for DA<T> {
    type Output = Result<DA<T>>;

    fn mul(self, rhs: T) -> Self::Output {
        &self * rhs
    }
}

/// Multiplication: &DA * T (scalar multiplication)
impl<T: DACoefficient> Mul<T> for &DA<T> {
    type Output = Result<DA<T>>;

    fn mul(self, rhs: T) -> Self::Output {
        let config = get_config()?;
        
        if rhs.abs() <= config.epsilon {
            return Ok(DA::zero());
        }

        let coeffs: HashMap<Monomial, T> = self.coeffs
            .iter()
            .map(|(&m, &c)| (m, c * rhs))
            .collect();

        Ok(DA { coeffs })
    }
}

// ============================================================================
// Note: T * DA and T * &DA impls removed due to Rust orphan rules
// Users should write DA * T instead (commutative anyway)
// ============================================================================

// ============================================================================
// Division
// ============================================================================

/// Division: DA / DA
impl<T: DACoefficient> Div<DA<T>> for DA<T> {
    type Output = Result<DA<T>>;

    fn div(self, rhs: DA<T>) -> Self::Output {
        &self / &rhs
    }
}

/// Division: &DA / &DA (preferred - avoids clones)
/// 
/// Uses an optimized order-by-order Taylor series division algorithm:
/// q_m = (f_m - Σ_{k<m} g_k * q_{m-k}) / g_0
/// 
/// Optimizations:
/// - Pre-sort divisor monomials once for deterministic iteration
/// - Early skip on zero coefficients
/// - Minimize hashmap lookups by caching frequently accessed values
/// - Process in order-by-order fashion ensuring all dependencies are satisfied
impl<T: DACoefficient> Div<&DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn div(self, rhs: &DA<T>) -> Self::Output {
        let g0 = rhs.constant_part();
        
        if g0.abs() < 1e-15 {
            return Err(anyhow::anyhow!("Division by zero"))
                .with_context(|| format!("...while dividing with {rhs:#?}"));
        }
        
        let config = get_config()?;
        let max_order = config.max_order;
        let epsilon = config.epsilon;
        let num_vars = config.num_vars;
        
        // Pre-compute inverse of g0 (one division vs many)
        let g0_inv = T::one() / g0;
        
        // Pre-sort divisor monomials for deterministic iteration (matches COSY)
        let mut sorted_g_monomials: Vec<_> = rhs.coeffs
            .iter()
            .filter(|(m, _)| **m != Monomial::constant())  // Skip constant term
            .map(|(m, c)| (*m, *c))
            .collect();
        sorted_g_monomials.sort_by_key(|(m, _)| *m);
        
        let mut result = DA::zero();
        
        // Process order by order - ensures q_{m-k} computed before q_m
        for order in 0..=max_order {
            let monomials_at_order = generate_monomials_of_order(order as u8, num_vars);
            
            for monomial in monomials_at_order {
                // Start with numerator coefficient
                let f_m = self.coeffs.get(&monomial).copied().unwrap_or(T::zero());
                
                // Early skip if numerator is zero and no contributions expected
                if f_m.abs() <= epsilon && order == 0 {
                    continue;
                }
                
                // Accumulate: Σ g_k * q_{m-k}
                let mut subtraction_sum = T::zero();
                
                for &(g_mono, g_coeff) in &sorted_g_monomials {
                    // Compute m - k (monomial difference)
                    let mut diff_exponents = [0u8; MAX_VARS];
                    let mut valid = true;
                    
                    for i in 0..MAX_VARS {
                        if monomial.exponents[i] >= g_mono.exponents[i] {
                            diff_exponents[i] = monomial.exponents[i] - g_mono.exponents[i];
                        } else {
                            valid = false;
                            break;
                        }
                    }
                    
                    if valid {
                        let diff_mono = Monomial::new(diff_exponents);
                        if let Some(&q_coeff) = result.coeffs.get(&diff_mono) {
                            // FMA: sum = g_k * q_{m-k} + sum (hardware-accelerated)
                            subtraction_sum = g_coeff.mul_add(q_coeff, subtraction_sum);
                        }
                    }
                }
                
                // Compute: q_m = (f_m - sum) / g_0
                // Using pre-computed inverse: q_m = (f_m - sum) * g0_inv
                let q_m = (f_m - subtraction_sum) * g0_inv;
                
                // Store if significant
                if q_m.abs() > epsilon {
                    result.coeffs.insert(monomial, q_m);
                }
            }
        }
        
        Ok(result)
    }
}

/// Generate all monomials of a specific total order for a given number of variables.
///
/// This is a helper function for division algorithm.
fn generate_monomials_of_order(target_order: u8, num_vars: usize) -> Vec<Monomial> {
    let mut result = Vec::new();
    let mut exponents = [0u8; MAX_VARS];
    
    // Use recursive generation
    fn generate_recursive(
        exponents: &mut [u8; MAX_VARS],
        var_index: usize,
        remaining_order: u8,
        num_vars: usize,
        result: &mut Vec<Monomial>,
    ) {
        if var_index >= num_vars {
            if remaining_order == 0 {
                result.push(Monomial::new(*exponents));
            }
            return;
        }
        
        // Try all possible exponents for this variable
        for exp in 0..=remaining_order {
            exponents[var_index] = exp;
            generate_recursive(exponents, var_index + 1, remaining_order - exp, num_vars, result);
        }
        exponents[var_index] = 0;  // Reset for next iteration
    }
    
    generate_recursive(&mut exponents, 0, target_order, num_vars, &mut result);
    result
}

/// Division: &DA / DA
impl<T: DACoefficient> Div<DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn div(self, rhs: DA<T>) -> Self::Output {
        self / &rhs
    }
}

/// Division: DA / &DA
impl<T: DACoefficient> Div<&DA<T>> for DA<T> {
    type Output = Result<DA<T>>;

    fn div(self, rhs: &DA<T>) -> Self::Output {
        &self / rhs
    }
}

/// Division: DA / T
impl<T: DACoefficient> Div<T> for DA<T> {
    type Output = Result<DA<T>>;

    fn div(self, rhs: T) -> Self::Output {
        &self / rhs
    }
}

/// Division: &DA / T (preferred - avoids clones)
impl<T: DACoefficient> Div<T> for &DA<T> {
    type Output = Result<DA<T>>;

    fn div(self, rhs: T) -> Self::Output {
        if rhs.abs() < 1e-15 {
            anyhow::bail!("Division by zero");
        }
        
        let coeffs: HashMap<Monomial, T> = self.coeffs
            .iter()
            .map(|(&m, &c)| (m, c / rhs))
            .collect();

        Ok(DA { coeffs })
    }
}

// ============================================================================
// Note: T / DA and T / &DA impls removed due to Rust orphan rules
// Users should write DA::from_coeff(value) / da instead
// ============================================================================

// ============================================================================
// Display & Debug
// ============================================================================

impl<T: DACoefficient> fmt::Debug for DA<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DA[")?;
        let mut sorted: Vec<_> = self.coeffs.iter().collect();
        sorted.sort_by_key(|(m, _)| *m);
        
        for (i, (monomial, coeff)) in sorted.iter().enumerate() {
            if i > 0 {
                write!(f, " + ")?;
            }
            write!(f, "{}*{}", coeff, monomial)?;
        }
        if sorted.is_empty() {
            write!(f, "0")?;
        }
        write!(f, "]")
    }
}

impl<T: DACoefficient> fmt::Display for DA<T> {
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
            write!(f, "{}", coeff)?;
            if monomial.total_order > 0 {
                write!(f, "*{}", monomial)?;
            }
        }
        Ok(())
    }
}

// ============================================================================
// Convenience Constructors
// ============================================================================

// These are kept on the generic DA<T> for specific coefficient types.
// The type aliases (DA = DA<f64>, CD = DA<Complex64>) are defined in mod.rs

impl DA<f64> {
    /// Create a real DA constant from an f64 value.
    pub fn constant(value: f64) -> Self {
        Self::from_coeff(value)
    }
}

impl DA<num_complex::Complex64> {
    /// Create a complex DA constant from a real value.
    pub fn constant(value: f64) -> Self {
        Self::from_coeff(num_complex::Complex64::new(value, 0.0))
    }

    /// Create a complex DA constant from a complex value.
    pub fn complex_constant(value: num_complex::Complex64) -> Self {
        Self::from_coeff(value)
    }

    /// Create a CD from a DA (real becomes real part, imaginary is zero).
    ///
    /// # Arguments
    /// * `da` - The DA to use as the real part
    ///
    /// # Returns
    /// A CD with the given real part and zero imaginary part
    pub fn from_da(da: &DA<f64>) -> Self {
        use num_complex::Complex64;
        let coeffs: HashMap<Monomial, Complex64> = da.coeffs_iter()
            .map(|(m, &v)| (*m, Complex64::new(v, 0.0)))
            .collect();
        
        Self { coeffs }
    }

    /// Create a CD from separate real and imaginary DA parts.
    ///
    /// # Arguments
    /// * `real` - The DA to use as the real part
    /// * `imag` - The DA to use as the imaginary part
    ///
    /// # Returns
    /// A CD combining the real and imaginary parts
    pub fn from_da_parts(real: &DA<f64>, imag: &DA<f64>) -> Self {
        use num_complex::Complex64;
        
        // Get all monomials from both real and imaginary parts
        let mut coeffs = HashMap::new();
        
        // Add real part coefficients
        for (monomial, &re_coeff) in real.coeffs_iter() {
            coeffs.insert(*monomial, Complex64::new(re_coeff, 0.0));
        }
        
        // Add/update imaginary part coefficients
        for (monomial, &im_coeff) in imag.coeffs_iter() {
            coeffs.entry(*monomial)
                .and_modify(|c| c.im = im_coeff)
                .or_insert(Complex64::new(0.0, im_coeff));
        }
        
        Self { coeffs }
    }

    /// Extract the real part of a complex DA as a real DA.
    pub fn real_part(&self) -> DA<f64> {
        let coeffs: HashMap<Monomial, f64> = self.coeffs_iter()
            .map(|(m, c)| (*m, c.re))
            .collect();
        DA { coeffs }
    }

    /// Extract the imaginary part of a complex DA as a real DA.
    pub fn imag_part(&self) -> DA<f64> {
        let coeffs: HashMap<Monomial, f64> = self.coeffs_iter()
            .map(|(m, c)| (*m, c.im))
            .collect();
        DA { coeffs }
    }
}