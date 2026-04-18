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

use crate::rosy_lib::taylor::DA;
use anyhow::{Result, bail};

#[cfg(feature = "nightly-simd")]
use std::simd::prelude::*;

#[cfg(feature = "nightly-simd")]
const LANES: usize = 4;

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
        bail!("POLVAL: NP ({}) must be >= NR ({})", np, nr);
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

/// Batch-evaluate NP polynomials at multiple particles simultaneously.
///
/// Each element of `a_array` is a VE (`Vec<f64>`) of particle values for one
/// coordinate axis. For example, `a_array\[0\]` holds x-values for all particles,
/// `a_array\[1\]` holds px-values, etc.
///
/// Results are written the same way: `r_array\[i\]` will hold the i-th result
/// component for all particles.
///
/// With `nightly-simd`: processes 4 particles per monomial using f64x4 SIMD.
pub fn rosy_polval_ve(
    _l: f64,
    p_array: &[DA],
    np: usize,
    a_array: &[Vec<f64>],
    na: usize,
    r_array: &mut Vec<Vec<f64>>,
    nr: usize,
) -> Result<()> {
    if np < nr {
        bail!("POLVAL: NP ({}) must be >= NR ({})", np, nr);
    }
    if a_array.len() < na {
        bail!(
            "POLVAL: argument array has {} elements but NA={}",
            a_array.len(),
            na
        );
    }

    let num_particles = if na > 0 { a_array[0].len() } else { 0 };

    while r_array.len() < nr {
        r_array.push(Vec::new());
    }

    for i in 0..nr {
        if i >= p_array.len() {
            bail!("POLVAL: polynomial array too short at index {}", i);
        }
        r_array[i].resize(num_particles, 0.0);
        evaluate_poly_batch(&p_array[i], a_array, na, num_particles, &mut r_array[i]);
    }

    Ok(())
}

/// Evaluate a single DA polynomial at all particles, writing results into `out`.
///
/// Iterates monomials once, processing particles in SIMD chunks of 4.
#[inline]
fn evaluate_poly_batch(
    poly: &DA,
    a_array: &[Vec<f64>],
    na: usize,
    num_particles: usize,
    out: &mut [f64],
) {
    // Zero the output
    out.iter_mut().for_each(|v| *v = 0.0);

    for (monomial, coeff) in poly.coeffs_iter().into_iter() {
        let exponents = &monomial.exponents;

        // Collect active variables (non-zero exponents) for this monomial
        let mut active_vars: [(usize, u8); 6] = [(0, 0); 6];
        let mut num_active = 0;
        for (var_idx, &exp) in exponents.iter().enumerate() {
            if exp != 0 && var_idx < na {
                active_vars[num_active] = (var_idx, exp);
                num_active += 1;
                if num_active >= 6 {
                    break;
                }
            }
        }

        // Constant monomial (no variables) — just add coefficient to all particles
        if num_active == 0 {
            for j in 0..num_particles {
                out[j] += coeff;
            }
            continue;
        }

        #[cfg(feature = "nightly-simd")]
        {
            let chunks = num_particles / LANES;
            let coeff_v = Simd::<f64, LANES>::splat(coeff);

            for c in 0..chunks {
                let base = c * LANES;
                let mut term = coeff_v;

                for a in 0..num_active {
                    let (var_idx, exp) = active_vars[a];
                    let vals = Simd::<f64, LANES>::from_slice(&a_array[var_idx][base..]);
                    term *= simd_powi(vals, exp);
                }

                let current = Simd::<f64, LANES>::from_slice(&out[base..]);
                (current + term).copy_to_slice(&mut out[base..base + LANES]);
            }

            // Scalar remainder
            for j in (chunks * LANES)..num_particles {
                let mut term = coeff;
                for a in 0..num_active {
                    let (var_idx, exp) = active_vars[a];
                    term *= scalar_powi(a_array[var_idx][j], exp);
                }
                out[j] += term;
            }
        }

        #[cfg(not(feature = "nightly-simd"))]
        {
            for j in 0..num_particles {
                let mut term = coeff;
                for a in 0..num_active {
                    let (var_idx, exp) = active_vars[a];
                    term *= scalar_powi(a_array[var_idx][j], exp);
                }
                out[j] += term;
            }
        }
    }
}

/// SIMD power: compute vals^exp for small exponents via repeated multiply.
#[cfg(feature = "nightly-simd")]
#[inline(always)]
fn simd_powi(vals: Simd<f64, LANES>, exp: u8) -> Simd<f64, LANES> {
    match exp {
        0 => Simd::<f64, LANES>::splat(1.0),
        1 => vals,
        2 => vals * vals,
        3 => vals * vals * vals,
        4 => {
            let v2 = vals * vals;
            v2 * v2
        }
        5 => {
            let v2 = vals * vals;
            v2 * v2 * vals
        }
        6 => {
            let v2 = vals * vals;
            v2 * v2 * v2
        }
        _ => {
            // General case via repeated squaring
            let mut result = Simd::<f64, LANES>::splat(1.0);
            let mut base = vals;
            let mut e = exp;
            while e > 0 {
                if e & 1 == 1 {
                    result *= base;
                }
                base *= base;
                e >>= 1;
            }
            result
        }
    }
}

/// Scalar power for small exponents.
#[inline(always)]
fn scalar_powi(val: f64, exp: u8) -> f64 {
    match exp {
        0 => 1.0,
        1 => val,
        2 => val * val,
        3 => val * val * val,
        4 => {
            let v2 = val * val;
            v2 * v2
        }
        5 => {
            let v2 = val * val;
            v2 * v2 * val
        }
        6 => {
            let v2 = val * val;
            v2 * v2 * v2
        }
        _ => val.powi(exp as i32),
    }
}

/// Evaluate a single DA polynomial at the given real-valued point.
///
/// For each monomial c * x1^e1 * x2^e2 * ... we substitute the values from
/// `args` (1-based variable indices mapped to 0-based slice positions) and
/// sum all contributions.
fn evaluate_da_at_re(poly: &DA, args: &[f64], na: usize) -> Result<f64> {
    let mut result = 0.0_f64;

    for (monomial, coeff) in poly.coeffs_iter().into_iter() {
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
