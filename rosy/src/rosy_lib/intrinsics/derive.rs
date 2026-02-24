use crate::rosy_lib::{DA, CD};
use crate::rosy_lib::taylor::{Monomial, get_config, MAX_VARS, DACoefficient};
use std::collections::HashMap;

/// Trait for derivation/anti-derivation of DA types.
/// Positive index = partial derivative, negative index = anti-derivative (integral).
pub trait RosyDerive {
    type Output;
    fn rosy_derive(&self, var_index: i64) -> anyhow::Result<Self::Output>;
}

/// Generic derivative implementation for DA<T>.
fn da_derivative<T: DACoefficient>(da: &crate::rosy_lib::taylor::da::DA<T>, var_idx: usize) -> anyhow::Result<crate::rosy_lib::taylor::da::DA<T>> {
    let config = get_config()?;
    let mut result_coeffs: HashMap<Monomial, T> = HashMap::new();
    
    // For each term c * x1^a1 * x2^a2 * ... * xn^an,
    // d/dx_i = a_i * c * x1^a1 * ... * x_i^(a_i - 1) * ... * xn^an
    for (monomial, &coeff) in da.coeffs_iter() {
        let exp_i = monomial.exponents[var_idx];
        if exp_i == 0 {
            continue; // This term vanishes under differentiation
        }
        
        let mut new_exponents = monomial.exponents;
        new_exponents[var_idx] -= 1;
        let new_monomial = Monomial::new(new_exponents);
        
        // Multiply coefficient by the exponent
        let mut factor = T::zero();
        for _ in 0..exp_i {
            factor += coeff;
        }
        // Actually: factor = coeff * exp_i (as T)
        // We need a cleaner way: build T from u8
        let scale = {
            let mut s = T::zero();
            let one = T::one();
            for _ in 0..exp_i {
                s += one;
            }
            s
        };
        let new_coeff = coeff * scale;
        
        if new_coeff.abs() > config.epsilon {
            *result_coeffs.entry(new_monomial).or_insert(T::zero()) += new_coeff;
        }
    }
    
    // Remove small coefficients
    result_coeffs.retain(|_, &mut coeff| coeff.abs() > config.epsilon);
    
    Ok(crate::rosy_lib::taylor::da::DA::from_coeffs(result_coeffs))
}

/// Generic anti-derivative (integral) implementation for DA<T>.
fn da_antiderivative<T: DACoefficient>(da: &crate::rosy_lib::taylor::da::DA<T>, var_idx: usize) -> anyhow::Result<crate::rosy_lib::taylor::da::DA<T>> {
    let config = get_config()?;
    let mut result_coeffs: HashMap<Monomial, T> = HashMap::new();
    
    // For each term c * x1^a1 * x2^a2 * ... * xn^an,
    // integral w.r.t. x_i = c/(a_i + 1) * x1^a1 * ... * x_i^(a_i + 1) * ... * xn^an
    for (monomial, &coeff) in da.coeffs_iter() {
        let exp_i = monomial.exponents[var_idx];
        let new_exp = exp_i + 1;
        
        let mut new_exponents = monomial.exponents;
        new_exponents[var_idx] = new_exp;
        let new_monomial = Monomial::new(new_exponents);
        
        // Skip if the new monomial exceeds the max order
        if !new_monomial.within_order(config.max_order) {
            continue;
        }
        
        // Divide coefficient by (exp_i + 1)
        let divisor = {
            let mut d = T::zero();
            let one = T::one();
            for _ in 0..new_exp {
                d += one;
            }
            d
        };
        let new_coeff = coeff / divisor;
        
        if new_coeff.abs() > config.epsilon {
            *result_coeffs.entry(new_monomial).or_insert(T::zero()) += new_coeff;
        }
    }
    
    // Remove small coefficients
    result_coeffs.retain(|_, &mut coeff| coeff.abs() > config.epsilon);
    
    Ok(crate::rosy_lib::taylor::da::DA::from_coeffs(result_coeffs))
}

impl RosyDerive for DA {
    type Output = DA;
    fn rosy_derive(&self, var_index: i64) -> anyhow::Result<Self::Output> {
        if var_index == 0 {
            anyhow::bail!("Derivation variable index cannot be 0");
        }
        
        if var_index > 0 {
            // Positive: partial derivative w.r.t. variable var_index
            let idx = (var_index as usize) - 1; // Convert to 0-based
            da_derivative(self, idx)
        } else {
            // Negative: anti-derivative (integral) w.r.t. variable |var_index|
            let idx = ((-var_index) as usize) - 1; // Convert to 0-based
            da_antiderivative(self, idx)
        }
    }
}

impl RosyDerive for CD {
    type Output = CD;
    fn rosy_derive(&self, var_index: i64) -> anyhow::Result<Self::Output> {
        if var_index == 0 {
            anyhow::bail!("Derivation variable index cannot be 0");
        }
        
        if var_index > 0 {
            let idx = (var_index as usize) - 1;
            da_derivative(self, idx)
        } else {
            let idx = ((-var_index) as usize) - 1;
            da_antiderivative(self, idx)
        }
    }
}
