//! DA (Differential Algebra) — Flat-array Taylor series with non-zero index tracking.
//!
//! This is the high-performance DA engine. Key design choices:
//!
//! - **Flat dense array**: Coefficients stored in a `Vec<T>` of length `num_monomials`,
//!   indexed by graded lexicographic monomial order. Eliminates HashMap overhead.
//!
//! - **Non-zero index tracking**: A `Vec<u32>` lists which coefficient indices are non-zero.
//!   Multiplication iterates only K² pairs (K = non-zero count) instead of N² (N = total
//!   monomials), giving 4–11× fewer iterations at typical beam physics sparsity.
//!
//! - **Precomputed multiplication table**: `mult_table[i*N + j]` = flat index of the
//!   product monomial, computed once at `init_taylor()`. Eliminates runtime exponent
//!   addition and hash lookups from the multiply inner loop.
//!
//! - **FMA in the hot loop**: `ci.mul_add(bj, result_k)` compiles to a single `vfmadd`
//!   instruction on modern x86, giving better accuracy and throughput.

use std::ops::{Add, Neg, Sub, Mul, Div, AddAssign};
use std::fmt;
use anyhow::{Result, Context};
use num_complex::Complex64;
use rustc_hash::FxHashMap;

use super::{Monomial, MAX_VARS};
use super::config::{get_runtime, get_config, MULT_INVALID, TaylorRuntime};

// ============================================================================
// Coefficient trait
// ============================================================================

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
    fn zero() -> Self;
    fn one() -> Self;
    fn abs(&self) -> f64;
}

impl DACoefficient for f64 {
    #[inline(always)] fn zero() -> Self { 0.0 }
    #[inline(always)] fn one() -> Self { 1.0 }
    #[inline(always)] fn abs(&self) -> f64 { f64::abs(*self) }
}

impl DACoefficient for Complex64 {
    #[inline(always)] fn zero() -> Self { Complex64::new(0.0, 0.0) }
    #[inline(always)] fn one() -> Self { Complex64::new(1.0, 0.0) }
    #[inline(always)] fn abs(&self) -> f64 { self.norm() }
}

/// Fused multiply-add: `self * a + b` in a single rounding operation.
pub trait MulAdd {
    fn mul_add(self, a: Self, b: Self) -> Self;
}

impl MulAdd for f64 {
    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        f64::mul_add(self, a, b)
    }
}

impl MulAdd for Complex64 {
    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        self * a + b
    }
}

// ============================================================================
// DA struct
// ============================================================================

/// Generic Taylor polynomial with flat-array storage and non-zero tracking.
///
/// `T` is the coefficient type: `f64` for real DA, `Complex64` for complex DA (CD).
///
/// # Representation
///
/// - `coeffs[i]` is the coefficient of monomial `monomial_list[i]` (from the runtime tables).
/// - `nonzero` lists the indices where `coeffs[i]` is significant (above epsilon).
/// - Index 0 is always the constant monomial.
/// - Indices 1..num_vars are the variable monomials x₁..xₙ.
#[derive(Clone)]
pub struct DA<T: DACoefficient> {
    /// Dense coefficient array, length = `num_monomials` from `TaylorRuntime`.
    pub coeffs: Vec<T>,
    /// Indices of non-zero coefficients (not necessarily sorted).
    pub nonzero: Vec<u32>,
}

impl<T: DACoefficient> PartialEq for DA<T> {
    fn eq(&self, other: &Self) -> bool {
        self.coeffs == other.coeffs
    }
}

// ============================================================================
// Helpers
// ============================================================================

/// Scan all coefficients, zero out those below epsilon or above max_order,
/// and return the list of non-zero indices.
fn build_nonzero_filtered<T: DACoefficient>(
    coeffs: &mut [T],
    epsilon: f64,
    max_order: u8,
    monomial_orders: &[u8],
) -> Vec<u32> {
    let mut nonzero = Vec::new();
    for (i, coeff) in coeffs.iter_mut().enumerate() {
        if monomial_orders[i] > max_order || coeff.abs() <= epsilon {
            *coeff = T::zero();
        } else {
            nonzero.push(i as u32);
        }
    }
    nonzero
}

// ============================================================================
// Basic methods
// ============================================================================

impl<T: DACoefficient> DA<T> {
    /// Create a zero DA (all coefficients zero).
    pub fn zero() -> Self {
        let rt = get_runtime().expect("Taylor system not initialized (call OV first)");
        Self {
            coeffs: vec![T::zero(); rt.num_monomials],
            nonzero: Vec::new(),
        }
    }

    /// Create a DA constant from a coefficient value.
    pub fn from_coeff(value: T) -> Self {
        let rt = get_runtime().expect("Taylor system not initialized");
        let epsilon = rt.config.epsilon;
        let mut coeffs = vec![T::zero(); rt.num_monomials];
        let mut nonzero = Vec::new();
        if value.abs() > epsilon {
            coeffs[0] = value;
            nonzero.push(0);
        }
        Self { coeffs, nonzero }
    }

    /// Create a DA variable (1-based index, matching COSY convention).
    ///
    /// `DA::variable(1)` creates x₁ (first variable), etc.
    pub fn variable(var_index: usize) -> Result<Self> {
        let rt = get_runtime()?;
        let config = rt.config;

        if var_index == 0 || var_index > config.num_vars {
            anyhow::bail!(
                "Variable index {} out of range [1, {}]",
                var_index, config.num_vars
            );
        }

        let flat_idx = rt.variable_indices[var_index - 1];
        let mut coeffs = vec![T::zero(); rt.num_monomials];
        coeffs[flat_idx as usize] = T::one();
        Ok(Self {
            coeffs,
            nonzero: vec![flat_idx],
        })
    }

    /// Get the constant (order-0) coefficient. Always at index 0.
    #[inline]
    pub fn constant_part(&self) -> T {
        self.coeffs[0]
    }

    /// Get a coefficient for a specific monomial (via runtime index lookup).
    pub fn get_coeff(&self, monomial: &Monomial) -> T {
        let rt = get_runtime().expect("Taylor system not initialized");
        if let Some(&idx) = rt.monomial_index.get(monomial) {
            self.coeffs[idx as usize]
        } else {
            T::zero()
        }
    }

    /// Set a coefficient for a specific monomial. Updates non-zero tracking.
    pub fn set_coeff(&mut self, monomial: Monomial, value: T) {
        let rt = get_runtime().expect("Taylor system not initialized");
        let epsilon = rt.config.epsilon;

        let idx = match rt.monomial_index.get(&monomial) {
            Some(&i) => i,
            None => return, // Monomial not in enumeration (e.g., beyond max_order)
        };

        let was_nz = self.coeffs[idx as usize].abs() > epsilon;
        self.coeffs[idx as usize] = value;
        let is_nz = value.abs() > epsilon;

        if is_nz && !was_nz {
            self.nonzero.push(idx);
        } else if !is_nz && was_nz {
            if let Some(pos) = self.nonzero.iter().position(|&i| i == idx) {
                self.nonzero.swap_remove(pos);
            }
            self.coeffs[idx as usize] = T::zero();
        }
    }

    /// Trim small coefficients below epsilon and rebuild the non-zero list.
    pub fn trim(&mut self) {
        let rt = get_runtime().expect("Taylor system not initialized");
        let epsilon = rt.config.epsilon;
        self.nonzero = build_nonzero_filtered(
            &mut self.coeffs, epsilon,
            rt.config.max_order as u8, &rt.monomial_orders,
        );
    }

    /// Number of non-zero coefficients.
    #[inline]
    pub fn num_terms(&self) -> usize {
        self.nonzero.len()
    }

    /// Check if this DA is effectively zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.nonzero.is_empty()
    }

    /// Create a DA from a HashMap of monomial→coefficient (compatibility bridge).
    pub fn from_coeffs(hash_coeffs: FxHashMap<Monomial, T>) -> Self {
        let rt = get_runtime().expect("Taylor system not initialized");
        let epsilon = rt.config.epsilon;
        let mut coeffs = vec![T::zero(); rt.num_monomials];
        for (mono, coeff) in hash_coeffs {
            if let Some(&idx) = rt.monomial_index.get(&mono) {
                if coeff.abs() > epsilon {
                    coeffs[idx as usize] = coeff;
                }
            }
        }
        let nonzero = build_nonzero_filtered(
            &mut coeffs, epsilon,
            rt.config.max_order as u8, &rt.monomial_orders,
        );
        Self { coeffs, nonzero }
    }

    /// Iterate non-zero entries as (Monomial, coefficient) pairs.
    ///
    /// Requires a runtime reference (caller should hold one to avoid repeated locking).
    pub fn coeffs_entries<'a>(&'a self, rt: &'a TaylorRuntime) -> impl Iterator<Item = (&'a Monomial, T)> + 'a {
        self.nonzero.iter().map(move |&i| (
            &rt.monomial_list[i as usize],
            self.coeffs[i as usize],
        ))
    }

    /// Iterate non-zero entries as (Monomial, coefficient) pairs (self-contained, allocates).
    ///
    /// Acquires the runtime lock internally. For hot-path code, prefer `coeffs_entries()`.
    pub fn coeffs_iter(&self) -> Vec<(Monomial, T)> {
        let rt = get_runtime().expect("Taylor system not initialized");
        self.nonzero.iter().map(|&i| (
            rt.monomial_list[i as usize],
            self.coeffs[i as usize],
        )).collect()
    }

    /// Add a scalar to the constant coefficient in-place.
    ///
    /// O(1) amortized. Used by Horner's method in transcendental functions.
    pub fn add_constant_in_place(&mut self, value: T) {
        let was_nz = self.coeffs[0].abs() > 1e-15;
        self.coeffs[0] = self.coeffs[0] + value;
        let is_nz = self.coeffs[0].abs() > 1e-15;

        if is_nz && !was_nz {
            self.nonzero.push(0);
        } else if !is_nz && was_nz {
            if let Some(pos) = self.nonzero.iter().position(|&i| i == 0) {
                self.nonzero.swap_remove(pos);
            }
            self.coeffs[0] = T::zero();
        }
    }
}

// ============================================================================
// Addition: DA + DA
// ============================================================================

impl<T: DACoefficient> Add<&DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn add(self, rhs: &DA<T>) -> Self::Output {
        let rt = get_runtime()?;
        let epsilon = rt.config.epsilon;
        let max_order = rt.config.max_order as u8;

        let mut coeffs = self.coeffs.clone();
        for &j in &rhs.nonzero {
            coeffs[j as usize] = coeffs[j as usize] + rhs.coeffs[j as usize];
        }
        let nonzero = build_nonzero_filtered(&mut coeffs, epsilon, max_order, &rt.monomial_orders);
        Ok(DA { coeffs, nonzero })
    }
}

impl<T: DACoefficient> Add<DA<T>> for DA<T> {
    type Output = Result<DA<T>>;
    fn add(self, rhs: DA<T>) -> Self::Output { &self + &rhs }
}

impl<T: DACoefficient> Add<&DA<T>> for DA<T> {
    type Output = Result<DA<T>>;
    fn add(self, rhs: &DA<T>) -> Self::Output { &self + rhs }
}

impl<T: DACoefficient> Add<DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;
    fn add(self, rhs: DA<T>) -> Self::Output { self + &rhs }
}

// Addition: DA + scalar
impl<T: DACoefficient> Add<T> for DA<T> {
    type Output = Result<DA<T>>;
    fn add(self, rhs: T) -> Self::Output { &self + rhs }
}

impl<T: DACoefficient> Add<T> for &DA<T> {
    type Output = Result<DA<T>>;
    fn add(self, rhs: T) -> Self::Output {
        let mut result = self.clone();
        result.add_constant_in_place(rhs);
        Ok(result)
    }
}

// ============================================================================
// Negation
// ============================================================================

impl<T: DACoefficient> Neg for DA<T> {
    type Output = DA<T>;
    fn neg(self) -> Self::Output { -&self }
}

impl<T: DACoefficient> Neg for &DA<T> {
    type Output = DA<T>;
    fn neg(self) -> Self::Output {
        DA {
            coeffs: self.coeffs.iter().map(|&c| -c).collect(),
            nonzero: self.nonzero.clone(),
        }
    }
}

// ============================================================================
// Subtraction: DA - DA (direct, avoids intermediate negation)
// ============================================================================

impl<T: DACoefficient> Sub<&DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn sub(self, rhs: &DA<T>) -> Self::Output {
        let rt = get_runtime()?;
        let epsilon = rt.config.epsilon;
        let max_order = rt.config.max_order as u8;

        let mut coeffs = self.coeffs.clone();
        for &j in &rhs.nonzero {
            coeffs[j as usize] = coeffs[j as usize] - rhs.coeffs[j as usize];
        }
        let nonzero = build_nonzero_filtered(&mut coeffs, epsilon, max_order, &rt.monomial_orders);
        Ok(DA { coeffs, nonzero })
    }
}

impl<T: DACoefficient> Sub<DA<T>> for DA<T> {
    type Output = Result<DA<T>>;
    fn sub(self, rhs: DA<T>) -> Self::Output { &self - &rhs }
}

impl<T: DACoefficient> Sub<&DA<T>> for DA<T> {
    type Output = Result<DA<T>>;
    fn sub(self, rhs: &DA<T>) -> Self::Output { &self - rhs }
}

impl<T: DACoefficient> Sub<DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;
    fn sub(self, rhs: DA<T>) -> Self::Output { self - &rhs }
}

// Subtraction: DA - scalar
impl<T: DACoefficient> Sub<T> for DA<T> {
    type Output = Result<DA<T>>;
    fn sub(self, rhs: T) -> Self::Output { &self - rhs }
}

impl<T: DACoefficient> Sub<T> for &DA<T> {
    type Output = Result<DA<T>>;
    fn sub(self, rhs: T) -> Self::Output {
        self + (-rhs)
    }
}

// ============================================================================
// Multiplication: DA * DA
// ============================================================================

impl<T: DACoefficient> Mul<&DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn mul(self, rhs: &DA<T>) -> Self::Output {
        let rt = get_runtime()?;
        let n = rt.num_monomials;
        let epsilon = rt.config.epsilon;
        let max_order = rt.config.max_order;
        let order_check = max_order < rt.init_order;

        let mut result = vec![T::zero(); n];

        if let Some(table) = &rt.mult_table {
            // Fast path: precomputed multiplication table
            for &i in &self.nonzero {
                let ci = self.coeffs[i as usize];
                let row = i as usize * n;
                for &j in &rhs.nonzero {
                    let k = table[row + j as usize];
                    if k != MULT_INVALID
                        && (!order_check || rt.monomial_orders[k as usize] as u32 <= max_order)
                    {
                        result[k as usize] = ci.mul_add(rhs.coeffs[j as usize], result[k as usize]);
                    }
                }
            }
        } else {
            // Fallback: compute product index on the fly
            for &i in &self.nonzero {
                let ci = self.coeffs[i as usize];
                for &j in &rhs.nonzero {
                    let product = rt.monomial_list[i as usize].multiply(&rt.monomial_list[j as usize]);
                    if product.within_order(max_order) {
                        if let Some(&k) = rt.monomial_index.get(&product) {
                            result[k as usize] = ci.mul_add(rhs.coeffs[j as usize], result[k as usize]);
                        }
                    }
                }
            }
        }

        let nonzero = build_nonzero_filtered(
            &mut result, epsilon, max_order as u8, &rt.monomial_orders,
        );
        Ok(DA { coeffs: result, nonzero })
    }
}

impl<T: DACoefficient> Mul<DA<T>> for DA<T> {
    type Output = Result<DA<T>>;
    fn mul(self, rhs: DA<T>) -> Self::Output { &self * &rhs }
}

impl<T: DACoefficient> Mul<&DA<T>> for DA<T> {
    type Output = Result<DA<T>>;
    fn mul(self, rhs: &DA<T>) -> Self::Output { &self * rhs }
}

impl<T: DACoefficient> Mul<DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;
    fn mul(self, rhs: DA<T>) -> Self::Output { self * &rhs }
}

// Multiplication: DA * scalar
impl<T: DACoefficient> Mul<T> for DA<T> {
    type Output = Result<DA<T>>;
    fn mul(self, rhs: T) -> Self::Output { &self * rhs }
}

impl<T: DACoefficient> Mul<T> for &DA<T> {
    type Output = Result<DA<T>>;
    fn mul(self, rhs: T) -> Self::Output {
        let rt = get_runtime()?;
        let epsilon = rt.config.epsilon;

        if rhs.abs() <= epsilon {
            return Ok(DA::zero());
        }

        let mut coeffs: Vec<T> = self.coeffs.iter().map(|&c| c * rhs).collect();
        let nonzero = build_nonzero_filtered(
            &mut coeffs, epsilon,
            rt.config.max_order as u8, &rt.monomial_orders,
        );
        Ok(DA { coeffs, nonzero })
    }
}

// ============================================================================
// Division: DA / DA (order-by-order algorithm)
// ============================================================================

impl<T: DACoefficient> Div<&DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn div(self, rhs: &DA<T>) -> Self::Output {
        let g0 = rhs.coeffs[0]; // constant part of divisor

        if g0.abs() < 1e-15 {
            return Err(anyhow::anyhow!("Division by zero"))
                .with_context(|| format!("...while dividing with {rhs:#?}"));
        }

        let rt = get_runtime()?;
        let n = rt.num_monomials;
        let max_order = rt.config.max_order;
        let epsilon = rt.config.epsilon;

        let g0_inv = T::one() / g0;

        // Collect non-zero divisor terms (excluding constant), sorted by index
        let mut g_entries: Vec<(u32, T)> = rhs.nonzero.iter()
            .filter(|&&i| i != 0)
            .map(|&i| (i, rhs.coeffs[i as usize]))
            .collect();
        g_entries.sort_unstable_by_key(|&(i, _)| i);

        let mut result = vec![T::zero(); n];

        // Process monomials in graded lex order (index 0..n).
        // Since monomials are stored in graded lex order, all dependencies
        // (lower-order quotient coefficients) are computed before they're needed.
        for m_idx in 0..n {
            if rt.monomial_orders[m_idx] as u32 > max_order {
                break;
            }

            let f_m = self.coeffs[m_idx];

            // Accumulate: Σ g_k * q_{m-k}
            let mut sum = T::zero();
            let mono_m = &rt.monomial_list[m_idx];

            for &(g_idx, g_coeff) in &g_entries {
                let mono_g = &rt.monomial_list[g_idx as usize];

                // Compute monomial difference: m - g (exponent-wise subtraction)
                let mut diff = [0u8; MAX_VARS];
                let mut valid = true;
                for v in 0..MAX_VARS {
                    if mono_m.exponents[v] >= mono_g.exponents[v] {
                        diff[v] = mono_m.exponents[v] - mono_g.exponents[v];
                    } else {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    let diff_mono = Monomial::new(diff);
                    if let Some(&diff_idx) = rt.monomial_index.get(&diff_mono) {
                        sum = g_coeff.mul_add(result[diff_idx as usize], sum);
                    }
                }
            }

            let q_m = (f_m - sum) * g0_inv;
            if q_m.abs() > epsilon {
                result[m_idx] = q_m;
            }
        }

        let nonzero = build_nonzero_filtered(
            &mut result, epsilon, max_order as u8, &rt.monomial_orders,
        );
        Ok(DA { coeffs: result, nonzero })
    }
}

impl<T: DACoefficient> Div<DA<T>> for DA<T> {
    type Output = Result<DA<T>>;
    fn div(self, rhs: DA<T>) -> Self::Output { &self / &rhs }
}

impl<T: DACoefficient> Div<DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;
    fn div(self, rhs: DA<T>) -> Self::Output { self / &rhs }
}

impl<T: DACoefficient> Div<&DA<T>> for DA<T> {
    type Output = Result<DA<T>>;
    fn div(self, rhs: &DA<T>) -> Self::Output { &self / rhs }
}

// Division: DA / scalar
impl<T: DACoefficient> Div<T> for DA<T> {
    type Output = Result<DA<T>>;
    fn div(self, rhs: T) -> Self::Output { &self / rhs }
}

impl<T: DACoefficient> Div<T> for &DA<T> {
    type Output = Result<DA<T>>;
    fn div(self, rhs: T) -> Self::Output {
        if rhs.abs() < 1e-15 {
            anyhow::bail!("Division by zero");
        }

        let coeffs: Vec<T> = self.coeffs.iter().map(|&c| c / rhs).collect();
        // Nonzero pattern preserved (dividing non-zero by non-zero stays non-zero)
        Ok(DA {
            coeffs,
            nonzero: self.nonzero.clone(),
        })
    }
}

// ============================================================================
// Display & Debug
// ============================================================================

impl<T: DACoefficient> fmt::Debug for DA<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rt = get_runtime().map_err(|_| fmt::Error)?;
        write!(f, "DA[")?;
        let mut entries: Vec<_> = self.nonzero.iter()
            .map(|&i| (i, self.coeffs[i as usize]))
            .collect();
        entries.sort_by_key(|&(i, _)| i);

        for (idx, (i, coeff)) in entries.iter().enumerate() {
            if idx > 0 { write!(f, " + ")?; }
            write!(f, "{}*{}", coeff, rt.monomial_list[*i as usize])?;
        }
        if entries.is_empty() {
            write!(f, "0")?;
        }
        write!(f, "]")
    }
}

impl<T: DACoefficient> fmt::Display for DA<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.nonzero.is_empty() {
            return write!(f, "0");
        }

        let rt = get_runtime().map_err(|_| fmt::Error)?;
        let mut entries: Vec<_> = self.nonzero.iter()
            .map(|&i| (i, self.coeffs[i as usize]))
            .collect();
        entries.sort_by_key(|&(i, _)| i);

        for (idx, &(i, coeff)) in entries.iter().enumerate() {
            if idx > 0 { write!(f, " + ")?; }
            let mono = &rt.monomial_list[i as usize];
            write!(f, "{}", coeff)?;
            if mono.total_order > 0 {
                write!(f, "*{}", mono)?;
            }
        }
        Ok(())
    }
}

// ============================================================================
// Convenience constructors
// ============================================================================

impl DA<f64> {
    /// Create a real DA constant from an f64 value.
    pub fn constant(value: f64) -> Self {
        Self::from_coeff(value)
    }
}

impl DA<Complex64> {
    /// Create a complex DA constant from a real value.
    pub fn constant(value: f64) -> Self {
        Self::from_coeff(Complex64::new(value, 0.0))
    }

    /// Create a complex DA constant from a complex value.
    pub fn complex_constant(value: Complex64) -> Self {
        Self::from_coeff(value)
    }

    /// Create a CD from a DA (real part; imaginary is zero).
    pub fn from_da(da: &DA<f64>) -> Self {
        let rt = get_runtime().expect("Taylor system not initialized");
        let mut coeffs = vec![Complex64::new(0.0, 0.0); rt.num_monomials];
        for &i in &da.nonzero {
            coeffs[i as usize] = Complex64::new(da.coeffs[i as usize], 0.0);
        }
        Self {
            coeffs,
            nonzero: da.nonzero.clone(),
        }
    }

    /// Create a CD from separate real and imaginary DA parts.
    pub fn from_da_parts(real: &DA<f64>, imag: &DA<f64>) -> Self {
        let rt = get_runtime().expect("Taylor system not initialized");
        let epsilon = rt.config.epsilon;
        let mut coeffs = vec![Complex64::new(0.0, 0.0); rt.num_monomials];

        for &i in &real.nonzero {
            coeffs[i as usize].re = real.coeffs[i as usize];
        }
        for &i in &imag.nonzero {
            coeffs[i as usize].im = imag.coeffs[i as usize];
        }

        let nonzero = build_nonzero_filtered(
            &mut coeffs, epsilon,
            rt.config.max_order as u8, &rt.monomial_orders,
        );
        Self { coeffs, nonzero }
    }

    /// Extract the real part as a real DA.
    pub fn real_part(&self) -> DA<f64> {
        let rt = get_runtime().expect("Taylor system not initialized");
        let epsilon = rt.config.epsilon;
        let mut coeffs: Vec<f64> = self.coeffs.iter().map(|c| c.re).collect();
        let nonzero = build_nonzero_filtered(
            &mut coeffs, epsilon,
            rt.config.max_order as u8, &rt.monomial_orders,
        );
        DA { coeffs, nonzero }
    }

    /// Extract the imaginary part as a real DA.
    pub fn imag_part(&self) -> DA<f64> {
        let rt = get_runtime().expect("Taylor system not initialized");
        let epsilon = rt.config.epsilon;
        let mut coeffs: Vec<f64> = self.coeffs.iter().map(|c| c.im).collect();
        let nonzero = build_nonzero_filtered(
            &mut coeffs, epsilon,
            rt.config.max_order as u8, &rt.monomial_orders,
        );
        DA { coeffs, nonzero }
    }
}
