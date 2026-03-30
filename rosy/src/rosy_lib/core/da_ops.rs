//! DA coefficient-level operations: DASCL, DASGN, DADER, DAINT, DANORO, DANORS.

use anyhow::{Result, Context, bail};

use crate::rosy_lib::taylor::{DA, get_runtime};
use crate::rosy_lib::taylor::config::DERIV_INVALID;

/// DASCL: Scale all coefficients of every DA element in the array by `scalar`.
///
/// Equivalent to `da[i] *= scalar` for all terms i.
pub fn rosy_dascl(da: &mut Vec<DA>, scalar: f64) -> Result<()> {
    for da_el in da.iter_mut() {
        *da_el = (&*da_el * scalar)
            .context("DASCL: failed to scale DA element")?;
    }
    Ok(())
}

/// DASGN: Negate all coefficients of every DA element in the array (sign flip).
///
/// Equivalent to `da[i] *= -1` for all terms i.
pub fn rosy_dasgn(da: &mut Vec<DA>) -> Result<()> {
    for da_el in da.iter_mut() {
        *da_el = -(&*da_el);
    }
    Ok(())
}

/// DADER: Differentiate every DA element in the array w.r.t. variable `var_index` (1-based).
///
/// For each monomial m_k with exponent e_v > 0 in variable v, the derivative contributes
/// `coeff_k * e_v` to the monomial with exponent v decremented by 1.
pub fn rosy_dader(da: &mut Vec<DA>, var_index: usize) -> Result<()> {
    // Acquire tables and release lock before building new DAs
    let (epsilon, _num_vars, deriv_target_v, deriv_exp_v) = {
        let rt = get_runtime().context("DADER requires DA to be initialized (call DAINI first)")?;
        let num_vars = rt.config.num_vars;
        if var_index == 0 || var_index > num_vars {
            bail!("DADER: variable index {} out of range [1, {}]", var_index, num_vars);
        }
        let v = var_index - 1;
        let n = rt.num_monomials;
        let base = v * n;
        let deriv_target_v = rt.deriv_target[base..base + n].to_vec();
        let deriv_exp_v    = rt.deriv_exponent[base..base + n].to_vec();
        (rt.config.epsilon, num_vars, deriv_target_v, deriv_exp_v)
    };

    for da_el in da.iter_mut() {
        // Snapshot: (flat_index, coefficient)
        let source: Vec<(usize, f64)> = da_el.nonzero.iter()
            .map(|&k| (k as usize, da_el.coeffs[k as usize]))
            .collect();

        let mut new_da = DA::zero();

        for (k, coeff_k) in source {
            let target = deriv_target_v[k];
            if target == DERIV_INVALID {
                continue; // exponent of v in monomial k is 0 → derivative is 0
            }
            let exp = deriv_exp_v[k] as f64;
            let new_coeff = coeff_k * exp;
            if new_coeff.abs() > epsilon {
                // Mapping is injective, so each target gets at most one contribution
                new_da.coeffs[target as usize] = new_coeff;
                new_da.nonzero.push(target);
            }
        }

        *da_el = new_da;
    }
    Ok(())
}

/// DAINT: Integrate every DA element in the array w.r.t. variable `var_index` (1-based).
///
/// For each monomial m_k with coefficient c_k, the integral contributes
/// `c_k / (e_v + 1)` to the monomial with exponent v incremented by 1.
/// Terms that would exceed the truncation order are dropped.
pub fn rosy_daint(da: &mut Vec<DA>, var_index: usize) -> Result<()> {
    // Acquire tables and release lock before building new DAs
    let (epsilon, _num_vars, integ_target_v, deriv_exp_v) = {
        let rt = get_runtime().context("DAINT requires DA to be initialized (call DAINI first)")?;
        let num_vars = rt.config.num_vars;
        if var_index == 0 || var_index > num_vars {
            bail!("DAINT: variable index {} out of range [1, {}]", var_index, num_vars);
        }
        let v = var_index - 1;
        let n = rt.num_monomials;
        let base = v * n;
        let integ_target_v = rt.integ_target[base..base + n].to_vec();
        // deriv_exp_v[t] = exponent of v in monomial t = (e_v_in_source + 1)
        let deriv_exp_v    = rt.deriv_exponent[base..base + n].to_vec();
        (rt.config.epsilon, num_vars, integ_target_v, deriv_exp_v)
    };

    for da_el in da.iter_mut() {
        let source: Vec<(usize, f64)> = da_el.nonzero.iter()
            .map(|&k| (k as usize, da_el.coeffs[k as usize]))
            .collect();

        let mut new_da = DA::zero();

        for (k, coeff_k) in source {
            let target = integ_target_v[k];
            if target == DERIV_INVALID {
                continue; // result would exceed truncation order
            }
            let t = target as usize;
            // The exponent of v in the target monomial equals (e_v_source + 1)
            let new_exp = deriv_exp_v[t] as f64;
            let new_coeff = coeff_k / new_exp;
            if new_coeff.abs() > epsilon {
                // Mapping is injective
                new_da.coeffs[t] = new_coeff;
                new_da.nonzero.push(target);
            }
        }

        *da_el = new_da;
    }
    Ok(())
}

/// DANORO: Remove all odd-order terms from every DA element in the array.
///
/// Keeps only terms whose total monomial order is even (0, 2, 4, …).
pub fn rosy_danoro(da: &mut Vec<DA>) -> Result<()> {
    let monomial_orders = {
        let rt = get_runtime().context("DANORO requires DA to be initialized (call DAINI first)")?;
        rt.monomial_orders.clone()
    };

    for da_el in da.iter_mut() {
        let mut i = 0;
        while i < da_el.nonzero.len() {
            let flat = da_el.nonzero[i] as usize;
            if monomial_orders[flat] % 2 == 1 {
                da_el.nonzero.swap_remove(i);
                da_el.coeffs[flat] = 0.0;
                // don't advance i — the swapped-in element needs to be checked
            } else {
                i += 1;
            }
        }
    }
    Ok(())
}

/// DANORS: Remove all coefficients whose absolute value is below `threshold`.
///
/// Keeps only terms with |c| >= threshold.
pub fn rosy_danors(da: &mut Vec<DA>, threshold: f64) -> Result<()> {
    for da_el in da.iter_mut() {
        let mut i = 0;
        while i < da_el.nonzero.len() {
            let flat = da_el.nonzero[i] as usize;
            if da_el.coeffs[flat].abs() < threshold {
                da_el.nonzero.swap_remove(i);
                da_el.coeffs[flat] = 0.0;
            } else {
                i += 1;
            }
        }
    }
    Ok(())
}
