//! POLVAL - polynomial evaluation / composition.
//!
//! `POLVAL L P NP A NA R NR;`
//!
//! Lets the polynomial described by NP DA vectors stored in the array P
//! act on the NA arguments A, and stores the NR results in R.
//!
//! In the normal case L == 1 (Horner evaluation).
//! The current implementation supports RE (f64) arguments (plain polynomial evaluation)
//! and returns an error for other argument types at runtime.

use anyhow::{Result, bail};
use crate::rosy_lib::taylor::DA;

/// Evaluate NP polynomials (stored in `p_array` as DA vectors) at the NA real
/// arguments in `a_array`, writing NR results into `r_array`.
///
/// # Arguments
/// * `_l`       - evaluation mode flag (1 = Horner; currently ignored, always Horner)
/// * `p_array`  - slice of NP DA polynomials
/// * `np`       - number of polynomials to evaluate
/// * `a_array`  - slice of NA real-valued arguments
/// * `na`       - number of arguments
/// * `r_array`  - output vector, must be large enough to hold NR results
/// * `nr`       - number of results to write
pub fn rosy_polval_re(
    _l: f64,
    p_array: &[DA],
    np: usize,
    a_array: &[f64],
    na: usize,
    r_array: &mut Vec<f64>,
    nr: usize,
) -> Result<()> {
    if np < nr {
        bail!(
            "POLVAL: NP ({}) must be >= NR ({})",
            np, nr
        );
    }

    // Ensure result vector is large enough
    while r_array.len() < nr {
        r_array.push(0.0);
    }

    for i in 0..nr {
        if i >= p_array.len() {
            bail!("POLVAL: polynomial array too short at index {}", i);
        }
        r_array[i] = evaluate_da_at_re(&p_array[i], a_array, na)?;
    }

    Ok(())
}

/// Evaluate a single DA polynomial at the given real-valued point.
///
/// For each monomial c * x1^e1 * x2^e2 * ... we substitute the values from
/// `args` (1-based variable indices mapped to 0-based slice positions) and
/// sum all contributions.
fn evaluate_da_at_re(
    poly: &DA,
    args: &[f64],
    na: usize,
) -> Result<f64> {
    let mut result = 0.0_f64;

    for (monomial, &coeff) in poly.coeffs_iter() {
        let exponents = &monomial.exponents;
        let mut term = coeff;

        for (var_idx, &exp) in exponents.iter().enumerate() {
            if exp == 0 {
                continue;
            }
            if var_idx >= na || var_idx >= args.len() {
                bail!(
                    "POLVAL: variable index {} out of range (NA={})",
                    var_idx + 1,
                    na
                );
            }
            term *= args[var_idx].powi(exp as i32);
        }

        result += term;
    }

    Ok(result)
}
