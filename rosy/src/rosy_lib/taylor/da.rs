//! DA (Differential Algebra) — Flat-array Taylor series with non-zero index tracking.
//!
//! All hot-path operations are O(K) where K = number of non-zero terms, not O(N)
//! where N = total monomials. This is critical for beam physics where DAs are
//! typically 20-50% dense — the flat array gives cache-friendly access while
//! non-zero tracking avoids touching zero entries.

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
/// All arithmetic operations only touch the K non-zero entries, not the full
/// N-element array. The `nonzero` list is the source of truth for which
/// indices contain valid data.
#[derive(Clone)]
pub struct DA<T: DACoefficient> {
    pub coeffs: Vec<T>,
    pub nonzero: Vec<u32>,
}

impl<T: DACoefficient> PartialEq for DA<T> {
    fn eq(&self, other: &Self) -> bool {
        // Compare only non-zero entries (both arrays should have zeros elsewhere)
        if self.nonzero.len() != other.nonzero.len() {
            return false;
        }
        // Both should have the same non-zero pattern
        for &i in &self.nonzero {
            if self.coeffs[i as usize] != other.coeffs[i as usize] {
                return false;
            }
        }
        for &i in &other.nonzero {
            if self.coeffs[i as usize] != other.coeffs[i as usize] {
                return false;
            }
        }
        true
    }
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

    #[inline]
    pub fn constant_part(&self) -> T {
        self.coeffs[0]
    }

    pub fn get_coeff(&self, monomial: &Monomial) -> T {
        let rt = get_runtime().expect("Taylor system not initialized");
        if let Some(&idx) = rt.monomial_index.get(monomial) {
            self.coeffs[idx as usize]
        } else {
            T::zero()
        }
    }

    pub fn set_coeff(&mut self, monomial: Monomial, value: T) {
        let rt = get_runtime().expect("Taylor system not initialized");
        let epsilon = rt.config.epsilon;

        let idx = match rt.monomial_index.get(&monomial) {
            Some(&i) => i,
            None => return,
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

    pub fn trim(&mut self) {
        let rt = get_runtime().expect("Taylor system not initialized");
        let epsilon = rt.config.epsilon;
        self.nonzero.retain(|&i| {
            if self.coeffs[i as usize].abs() <= epsilon {
                false
            } else {
                true
            }
        });
    }

    #[inline]
    pub fn num_terms(&self) -> usize {
        self.nonzero.len()
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.nonzero.is_empty()
    }

    pub fn from_coeffs(hash_coeffs: FxHashMap<Monomial, T>) -> Self {
        let rt = get_runtime().expect("Taylor system not initialized");
        let epsilon = rt.config.epsilon;
        let mut coeffs = vec![T::zero(); rt.num_monomials];
        let mut nonzero = Vec::new();
        for (mono, coeff) in hash_coeffs {
            if let Some(&idx) = rt.monomial_index.get(&mono) {
                if coeff.abs() > epsilon {
                    coeffs[idx as usize] = coeff;
                    nonzero.push(idx);
                }
            }
        }
        Self { coeffs, nonzero }
    }

    pub fn coeffs_entries<'a>(&'a self, rt: &'a TaylorRuntime) -> impl Iterator<Item = (&'a Monomial, T)> + 'a {
        self.nonzero.iter().map(move |&i| (
            &rt.monomial_list[i as usize],
            self.coeffs[i as usize],
        ))
    }

    pub fn coeffs_iter(&self) -> Vec<(Monomial, T)> {
        let rt = get_runtime().expect("Taylor system not initialized");
        self.nonzero.iter().map(|&i| (
            rt.monomial_list[i as usize],
            self.coeffs[i as usize],
        )).collect()
    }

    /// O(1) amortized. Used by Horner's method.
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
// Addition: O(K_self + K_rhs) — no full-array clone or scan
// ============================================================================

impl<T: DACoefficient> Add<&DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn add(self, rhs: &DA<T>) -> Self::Output {
        let rt = get_runtime()?;
        let n = rt.num_monomials;
        let epsilon = rt.config.epsilon;
        let max_order = rt.config.max_order as u8;
        let orders = &rt.monomial_orders;

        // Bitset to track which indices belong to self
        let words = (n + 63) / 64;
        let mut self_set = vec![0u64; words];

        // Start with a zeroed array, copy only self's nonzero entries
        let mut coeffs = vec![T::zero(); n];
        for &i in &self.nonzero {
            let iu = i as usize;
            if orders[iu] <= max_order {
                coeffs[iu] = self.coeffs[iu];
                self_set[iu / 64] |= 1u64 << (iu % 64);
            }
        }

        // Add rhs's nonzero entries
        for &j in &rhs.nonzero {
            let ju = j as usize;
            if orders[ju] <= max_order {
                coeffs[ju] = coeffs[ju] + rhs.coeffs[ju];
            }
        }

        // Build nonzero from self's entries (may have cancelled)
        let mut nonzero = Vec::with_capacity(self.nonzero.len() + rhs.nonzero.len());
        for &i in &self.nonzero {
            let iu = i as usize;
            if orders[iu] <= max_order && coeffs[iu].abs() > epsilon {
                nonzero.push(i);
            } else {
                coeffs[iu] = T::zero();
            }
        }
        // Add rhs-only entries (not in self)
        for &j in &rhs.nonzero {
            let ju = j as usize;
            if self_set[ju / 64] & (1u64 << (ju % 64)) == 0 {
                if orders[ju] <= max_order && coeffs[ju].abs() > epsilon {
                    nonzero.push(j);
                } else {
                    coeffs[ju] = T::zero();
                }
            }
        }

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

// Addition: DA + scalar — O(K) clone + O(1) constant update
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
// Negation: O(K) — only negate nonzero entries
// ============================================================================

impl<T: DACoefficient> Neg for DA<T> {
    type Output = DA<T>;
    fn neg(self) -> Self::Output { -&self }
}

impl<T: DACoefficient> Neg for &DA<T> {
    type Output = DA<T>;
    fn neg(self) -> Self::Output {
        let rt = get_runtime().expect("Taylor system not initialized");
        let mut coeffs = vec![T::zero(); rt.num_monomials];
        for &i in &self.nonzero {
            coeffs[i as usize] = -self.coeffs[i as usize];
        }
        DA {
            coeffs,
            nonzero: self.nonzero.clone(),
        }
    }
}

// ============================================================================
// Subtraction: O(K_self + K_rhs) — same as addition, no intermediate negation
// ============================================================================

impl<T: DACoefficient> Sub<&DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn sub(self, rhs: &DA<T>) -> Self::Output {
        let rt = get_runtime()?;
        let n = rt.num_monomials;
        let epsilon = rt.config.epsilon;
        let max_order = rt.config.max_order as u8;
        let orders = &rt.monomial_orders;

        let words = (n + 63) / 64;
        let mut self_set = vec![0u64; words];

        let mut coeffs = vec![T::zero(); n];
        for &i in &self.nonzero {
            let iu = i as usize;
            if orders[iu] <= max_order {
                coeffs[iu] = self.coeffs[iu];
                self_set[iu / 64] |= 1u64 << (iu % 64);
            }
        }

        for &j in &rhs.nonzero {
            let ju = j as usize;
            if orders[ju] <= max_order {
                coeffs[ju] = coeffs[ju] - rhs.coeffs[ju];
            }
        }

        let mut nonzero = Vec::with_capacity(self.nonzero.len() + rhs.nonzero.len());
        for &i in &self.nonzero {
            let iu = i as usize;
            if orders[iu] <= max_order && coeffs[iu].abs() > epsilon {
                nonzero.push(i);
            } else {
                coeffs[iu] = T::zero();
            }
        }
        for &j in &rhs.nonzero {
            let ju = j as usize;
            if self_set[ju / 64] & (1u64 << (ju % 64)) == 0 {
                if orders[ju] <= max_order && coeffs[ju].abs() > epsilon {
                    nonzero.push(j);
                } else {
                    coeffs[ju] = T::zero();
                }
            }
        }

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
// Multiplication: O(K²) inner loop + O(N) zero-init + bitset nonzero tracking
// ============================================================================

impl<T: DACoefficient> Mul<&DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn mul(self, rhs: &DA<T>) -> Self::Output {
        let rt = get_runtime()?;
        let n = rt.num_monomials;
        let epsilon = rt.config.epsilon;
        let max_order = rt.config.max_order;
        let order_check = max_order < rt.init_order;

        // Zero-init is needed for FMA accumulation (result[k] starts at 0)
        let mut result = vec![T::zero(); n];

        // Bitset to track which result indices were written
        let words = (n + 63) / 64;
        let mut written = vec![0u64; words];

        if let Some(table) = &rt.mult_table {
            for &i in &self.nonzero {
                let ci = self.coeffs[i as usize];
                let row = i as usize * n;
                for &j in &rhs.nonzero {
                    let k = table[row + j as usize];
                    if k != MULT_INVALID
                        && (!order_check || rt.monomial_orders[k as usize] as u32 <= max_order)
                    {
                        let ku = k as usize;
                        result[ku] = ci.mul_add(rhs.coeffs[j as usize], result[ku]);
                        written[ku / 64] |= 1u64 << (ku % 64);
                    }
                }
            }
        } else {
            for &i in &self.nonzero {
                let ci = self.coeffs[i as usize];
                for &j in &rhs.nonzero {
                    let product = rt.monomial_list[i as usize].multiply(&rt.monomial_list[j as usize]);
                    if product.within_order(max_order) {
                        if let Some(&k) = rt.monomial_index.get(&product) {
                            let ku = k as usize;
                            result[ku] = ci.mul_add(rhs.coeffs[j as usize], result[ku]);
                            written[ku / 64] |= 1u64 << (ku % 64);
                        }
                    }
                }
            }
        }

        // Build nonzero from bitset — O(words + K_result) instead of O(N)
        let mut nonzero = Vec::new();
        for word_idx in 0..words {
            let mut word = written[word_idx];
            while word != 0 {
                let bit = word.trailing_zeros() as usize;
                let idx = word_idx * 64 + bit;
                if idx < n {
                    if result[idx].abs() > epsilon {
                        nonzero.push(idx as u32);
                    } else {
                        result[idx] = T::zero();
                    }
                }
                word &= word - 1;
            }
        }

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

// Scalar multiply: O(K) — only scale nonzero entries
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

        let mut coeffs = vec![T::zero(); rt.num_monomials];
        for &i in &self.nonzero {
            coeffs[i as usize] = self.coeffs[i as usize] * rhs;
        }
        Ok(DA {
            coeffs,
            nonzero: self.nonzero.clone(),
        })
    }
}

// ============================================================================
// Division: order-by-order algorithm (not hot-path)
// ============================================================================

impl<T: DACoefficient> Div<&DA<T>> for &DA<T> {
    type Output = Result<DA<T>>;

    fn div(self, rhs: &DA<T>) -> Self::Output {
        let g0 = rhs.coeffs[0];

        if g0.abs() < 1e-15 {
            return Err(anyhow::anyhow!("Division by zero"))
                .with_context(|| format!("...while dividing with {rhs:#?}"));
        }

        let rt = get_runtime()?;
        let n = rt.num_monomials;
        let max_order = rt.config.max_order;
        let epsilon = rt.config.epsilon;

        let g0_inv = T::one() / g0;

        let mut g_entries: Vec<(u32, T)> = rhs.nonzero.iter()
            .filter(|&&i| i != 0)
            .map(|&i| (i, rhs.coeffs[i as usize]))
            .collect();
        g_entries.sort_unstable_by_key(|&(i, _)| i);

        let mut result = vec![T::zero(); n];
        let mut nonzero = Vec::new();

        for m_idx in 0..n {
            if rt.monomial_orders[m_idx] as u32 > max_order {
                break;
            }

            let f_m = self.coeffs[m_idx];
            let mut sum = T::zero();
            let mono_m = &rt.monomial_list[m_idx];

            for &(g_idx, g_coeff) in &g_entries {
                let mono_g = &rt.monomial_list[g_idx as usize];

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
                nonzero.push(m_idx as u32);
            }
        }

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

// Scalar division: O(K) — only divide nonzero entries
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

        let rt = get_runtime().expect("Taylor system not initialized");
        let mut coeffs = vec![T::zero(); rt.num_monomials];
        for &i in &self.nonzero {
            coeffs[i as usize] = self.coeffs[i as usize] / rhs;
        }
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
    pub fn constant(value: f64) -> Self {
        Self::from_coeff(value)
    }
}

impl DA<Complex64> {
    pub fn constant(value: f64) -> Self {
        Self::from_coeff(Complex64::new(value, 0.0))
    }

    pub fn complex_constant(value: Complex64) -> Self {
        Self::from_coeff(value)
    }

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

    pub fn from_da_parts(real: &DA<f64>, imag: &DA<f64>) -> Self {
        let rt = get_runtime().expect("Taylor system not initialized");
        let n = rt.num_monomials;
        let epsilon = rt.config.epsilon;
        let mut coeffs = vec![Complex64::new(0.0, 0.0); n];

        // Use bitset to build union nonzero
        let words = (n + 63) / 64;
        let mut nz_set = vec![0u64; words];

        for &i in &real.nonzero {
            coeffs[i as usize].re = real.coeffs[i as usize];
            nz_set[i as usize / 64] |= 1u64 << (i as usize % 64);
        }
        for &i in &imag.nonzero {
            coeffs[i as usize].im = imag.coeffs[i as usize];
            nz_set[i as usize / 64] |= 1u64 << (i as usize % 64);
        }

        let mut nonzero = Vec::new();
        for word_idx in 0..words {
            let mut word = nz_set[word_idx];
            while word != 0 {
                let bit = word.trailing_zeros() as usize;
                let idx = word_idx * 64 + bit;
                if idx < n && coeffs[idx].abs() > epsilon {
                    nonzero.push(idx as u32);
                }
                word &= word - 1;
            }
        }
        Self { coeffs, nonzero }
    }

    pub fn real_part(&self) -> DA<f64> {
        let rt = get_runtime().expect("Taylor system not initialized");
        let epsilon = rt.config.epsilon;
        let mut coeffs = vec![0.0f64; rt.num_monomials];
        let mut nonzero = Vec::new();
        for &i in &self.nonzero {
            let re = self.coeffs[i as usize].re;
            if re.abs() > epsilon {
                coeffs[i as usize] = re;
                nonzero.push(i);
            }
        }
        DA { coeffs, nonzero }
    }

    pub fn imag_part(&self) -> DA<f64> {
        let rt = get_runtime().expect("Taylor system not initialized");
        let epsilon = rt.config.epsilon;
        let mut coeffs = vec![0.0f64; rt.num_monomials];
        let mut nonzero = Vec::new();
        for &i in &self.nonzero {
            let im = self.coeffs[i as usize].im;
            if im.abs() > epsilon {
                coeffs[i as usize] = im;
                nonzero.push(i);
            }
        }
        DA { coeffs, nonzero }
    }
}
