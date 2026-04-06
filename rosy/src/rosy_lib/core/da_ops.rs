//! DA coefficient-level operations: DASCL, DASGN, DADER, DAINT, DANORO, DANORS,
//! DAFSET, DAFILT, DARAN, DAGMD, DACODE, DAFLO, CDFLO.

use anyhow::{Result, Context, bail};
use num_complex::Complex64;

use crate::rosy_lib::taylor::{DA, CD, get_runtime, get_filter_da, set_filter_da};
use crate::rosy_lib::taylor::da::{DA as GenericDA, DACoefficient};
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

/// DAFSET: Set the DA filtering template used by DAFILT.
///
/// Pass `template = 0.0` (scalar 0) to disable filtering. Otherwise the
/// single DA value is used as the template for all components.
/// In Rosy, `template_da` is a `Vec<DA>` with at least one element;
/// pass an empty vec to disable.
pub fn rosy_dafset(template_da: Vec<DA>) -> Result<()> {
    if template_da.is_empty() || (template_da.len() == 1 && template_da[0].is_zero()) {
        set_filter_da(None)
    } else {
        set_filter_da(Some(template_da))
    }
}

/// DAFILT: Filter `input` through the template set by DAFSET, writing to `result`.
///
/// For each monomial index, a coefficient is kept only if the corresponding
/// monomial is nonzero in the template component (component 0 of the template
/// is used as a universal mask).
pub fn rosy_dafilt(input: &Vec<DA>, result: &mut Vec<DA>) -> Result<()> {
    let filter = get_filter_da()?;

    match filter {
        None => {
            // No filter set — copy input to result unchanged
            for (r, src) in result.iter_mut().zip(input.iter()) {
                *r = src.clone();
            }
        }
        Some(template) => {
            let mask = &template[0]; // use first template component as the monomial mask
            for (r, src) in result.iter_mut().zip(input.iter()) {
                // Keep only monomials that are nonzero in the mask
                let mut new_da = DA::zero();
                for &k in &src.nonzero {
                    if mask.coeffs[k as usize].abs() > 0.0 {
                        new_da.coeffs[k as usize] = src.coeffs[k as usize];
                        new_da.nonzero.push(k);
                    }
                }
                *r = new_da;
            }
        }
    }
    Ok(())
}

/// DARAN: Fill every DA element in `da` with random coefficients in [-1, 1].
///
/// `sparsity` is the fraction of monomials that will be set nonzero
/// (0.0 = all zero, 1.0 = all filled). Uses the global Rosy RNG.
pub fn rosy_daran(da: &mut Vec<DA>, sparsity: f64) -> Result<()> {
    let num_monomials = {
        let rt = get_runtime().context("DARAN requires DA to be initialized (call DAINI first)")?;
        rt.num_monomials
    };

    let sparsity = sparsity.clamp(0.0, 1.0);

    for da_el in da.iter_mut() {
        *da_el = DA::zero();
        for k in 0..num_monomials {
            if crate::rosy_lib::core::rng::rng_f64() < sparsity {
                let val = crate::rosy_lib::core::rng::rng_f64_symmetric();
                da_el.coeffs[k] = val;
                da_el.nonzero.push(k as u32);
            }
        }
    }
    Ok(())
}

/// DAGMD: Compute the Lie derivative ∇g · f (gradient of g dotted with vector field f).
///
/// `result = Σᵢ (∂g/∂xᵢ) * f[i]`
///
/// Arguments:
/// - `g`: single-component DA (scalar field)
/// - `f`: array of DAs (vector field, `dim` components)
/// - `result`: single-component DA output
/// - `dim`: number of components of f
pub fn rosy_dagmd(g: &Vec<DA>, f: &Vec<DA>, result: &mut Vec<DA>, dim: usize) -> Result<()> {
    let (epsilon, num_vars, deriv_targets, deriv_exponents, n) = {
        let rt = get_runtime().context("DAGMD requires DA to be initialized (call DAINI first)")?;
        let num_vars = rt.config.num_vars;
        (
            rt.config.epsilon,
            num_vars,
            rt.deriv_target.clone(),
            rt.deriv_exponent.clone(),
            rt.num_monomials,
        )
    };

    if dim > num_vars {
        bail!("DAGMD: dim ({}) exceeds number of DA variables ({})", dim, num_vars);
    }
    if f.len() < dim {
        bail!("DAGMD: f array has {} elements but dim={}", f.len(), dim);
    }

    // g is a single-component DA; we compute ∂g/∂xᵢ for each i, multiply by f[i], accumulate
    let g0 = g.get(0).ok_or_else(|| anyhow::anyhow!("DAGMD: g is empty"))?;

    let mut acc = DA::zero();

    for i in 0..dim {
        // Compute ∂g/∂xᵢ (derivative w.r.t. variable i+1, 0-based: i)
        let v = i;
        let base = v * n;
        let deriv_target_v = &deriv_targets[base..base + n];
        let deriv_exp_v    = &deriv_exponents[base..base + n];

        let mut dg_dxi = DA::zero();
        for &k in &g0.nonzero {
            let target = deriv_target_v[k as usize];
            if target == DERIV_INVALID { continue; }
            let exp = deriv_exp_v[k as usize] as f64;
            let new_coeff = g0.coeffs[k as usize] * exp;
            if new_coeff.abs() > epsilon {
                dg_dxi.coeffs[target as usize] = new_coeff;
                dg_dxi.nonzero.push(target);
            }
        }

        // Multiply dg_dxi by f[i] and add to accumulator
        let fi = &f[i];
        let product = (&dg_dxi * fi).context("DAGMD: multiplication failed")?;
        acc = (&acc + &product).context("DAGMD: addition failed")?;
    }

    if let Some(r) = result.get_mut(0) {
        *r = acc;
    } else {
        result.push(acc);
    }

    Ok(())
}

/// DACODE: Decode monomial indices into exponent arrays.
///
/// For each monomial index m (0-based internally, 1-based in ROSY), writes the
/// exponent vector into `result[m]`. The `params` vector should contain at least
/// `[order, num_vars]` for validation against the current DAINI setup.
///
/// `result` is a 2D RE array: `result[m][v]` = exponent of variable v+1 in monomial m.
pub fn rosy_dacode(params: &Vec<f64>, size: usize, result: &mut Vec<Vec<f64>>) -> Result<()> {
    let (num_monomials, num_vars, monomial_list) = {
        let rt = get_runtime().context("DACODE requires DA to be initialized (call DAINI first)")?;
        (rt.num_monomials, rt.config.num_vars, rt.monomial_list.clone())
    };

    // Validate params against current setup
    if params.len() >= 2 {
        let p_order = params[0] as u32;
        let p_vars = params[1] as usize;
        let rt = get_runtime()?;
        if p_order != rt.config.max_order {
            bail!("DACODE: params order ({}) does not match current DA order ({})", p_order, rt.config.max_order);
        }
        if p_vars != num_vars {
            bail!("DACODE: params num_vars ({}) does not match current DA num_vars ({})", p_vars, num_vars);
        }
    }

    let count = size.min(num_monomials).min(result.len());

    for m in 0..count {
        let mono = &monomial_list[m];
        let inner = &mut result[m];
        let vlen = inner.len().min(num_vars);
        for v in 0..vlen {
            inner[v] = mono.exponents[v] as f64;
        }
    }

    Ok(())
}

// ============================================================================
// Generic Lie series flow (shared by DAFLO and CDFLO)
// ============================================================================

/// Compute the Lie derivative L_f(g) = ∇g · f = Σᵢ (∂g/∂xᵢ) · fᵢ
/// for a single scalar DA `g` and vector field `f`.
///
/// Generic over coefficient type T (f64 or Complex64).
fn lie_derivative_scalar<T: DACoefficient>(
    g: &GenericDA<T>,
    f: &[GenericDA<T>],
    dim: usize,
    epsilon: f64,
    deriv_targets: &[u32],
    deriv_exponents: &[u8],
    n: usize,
) -> Result<GenericDA<T>> {
    let mut acc = GenericDA::<T>::zero();

    for i in 0..dim {
        let base = i * n;
        let deriv_target_v = &deriv_targets[base..base + n];
        let deriv_exp_v = &deriv_exponents[base..base + n];

        // Compute ∂g/∂xᵢ
        let mut dg_dxi = GenericDA::<T>::zero();
        for &k in &g.nonzero {
            let target = deriv_target_v[k as usize];
            if target == DERIV_INVALID { continue; }
            let exp = T::from_usize(deriv_exp_v[k as usize] as usize);
            let new_coeff = g.coeffs[k as usize] * exp;
            if new_coeff.abs() > epsilon {
                dg_dxi.coeffs[target as usize] = new_coeff;
                dg_dxi.nonzero.push(target);
            }
        }

        // Multiply ∂g/∂xᵢ by f[i] and accumulate
        let product = (&dg_dxi * &f[i]).context("Lie derivative: multiplication failed")?;
        acc = (&acc + &product).context("Lie derivative: addition failed")?;
    }

    Ok(acc)
}

/// Generic Lie series flow: computes exp(L_f)(ic) for time step 1.
///
/// For each component i of the initial condition, iterates:
///   term_0 = ic[i]
///   term_k = (1/k) · L_f(term_{k-1})
///   result[i] = Σ term_k
///
/// Converges to machine accuracy for polynomial vector fields;
/// DA truncation guarantees termination for any smooth f.
fn flow_impl<T: DACoefficient>(
    rhs: &[GenericDA<T>],
    ic: &[GenericDA<T>],
    result: &mut Vec<GenericDA<T>>,
    dim: usize,
) -> Result<()> {
    let (epsilon, num_vars, deriv_targets, deriv_exponents, n) = {
        let rt = get_runtime().context("Flow requires DA to be initialized (call DAINI first)")?;
        (
            rt.config.epsilon,
            rt.config.num_vars,
            rt.deriv_target.clone(),
            rt.deriv_exponent.clone(),
            rt.num_monomials,
        )
    };

    if dim > num_vars {
        bail!("Flow: dim ({}) exceeds number of DA variables ({})", dim, num_vars);
    }
    if rhs.len() < dim {
        bail!("Flow: rhs array has {} elements but dim={}", rhs.len(), dim);
    }
    if ic.len() < dim {
        bail!("Flow: initial condition has {} elements but dim={}", ic.len(), dim);
    }

    const MAX_ITER: usize = 200;

    for i in 0..dim {
        let mut term = ic[i].clone();
        let mut sum = term.clone();

        for k in 1..=MAX_ITER {
            // term_k = (1/k) · L_f(term_{k-1})
            let lie = lie_derivative_scalar(
                &term, &rhs[..dim], dim, epsilon,
                &deriv_targets, &deriv_exponents, n,
            )?;
            let scale = T::one() / T::from_usize(k);
            term = (&lie * scale).context("Flow: scaling failed")?;

            // Check convergence: max |coefficient| in the new term
            let term_norm: f64 = term.nonzero.iter()
                .map(|&idx| term.coeffs[idx as usize].abs())
                .fold(0.0f64, f64::max);

            if term_norm < epsilon {
                break;
            }

            sum = (&sum + &term).context("Flow: accumulation failed")?;
        }

        if let Some(r) = result.get_mut(i) {
            *r = sum;
        } else {
            result.push(sum);
        }
    }

    Ok(())
}

/// DAFLO: Compute the real DA flow of x' = f(x) for time step 1.
///
/// Arguments:
/// - `rhs`: array of DAs representing the vector field f (dim components)
/// - `ic`: initial condition DA array (dim components)
/// - `result`: output DA array (dim components)
/// - `dim`: dimension of the system
pub fn rosy_daflo(rhs: &Vec<DA>, ic: &Vec<DA>, result: &mut Vec<DA>, dim: usize) -> Result<()> {
    flow_impl(rhs, ic, result, dim)
}

/// CDFLO: Compute the complex DA flow of x' = f(x) for time step 1.
///
/// Same as DAFLO but with complex DA (CD) coefficients.
pub fn rosy_cdflo(rhs: &Vec<CD>, ic: &Vec<CD>, result: &mut Vec<CD>, dim: usize) -> Result<()> {
    flow_impl(rhs, ic, result, dim)
}
