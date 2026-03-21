use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, VE, DA};

/// Type registry for ASIN intrinsic function.
///
/// According to COSY INFINITY manual, ASIN supports:
/// - RE -> RE
/// - VE -> VE (elementwise)
/// - DA -> DA (Taylor composition)
///
/// Note: CM is NOT supported for ASIN in COSY.
pub const ASIN_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "0.5"),
    IntrinsicTypeRule::new("VE", "VE", "0.1&0.2&0.3"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1)"),
];

/// Get the return type of ASIN for a given input type.
pub fn get_return_type(input: &RosyType) -> Option<RosyType> {
    let registry: HashMap<RosyType, RosyType> = {
        let mut m = HashMap::new();
        let all = vec![
            (RosyType::RE(), RosyType::RE()),
            (RosyType::VE(), RosyType::VE()),
            (RosyType::DA(), RosyType::DA()),
        ];
        for (input_type, result_type) in all {
            m.insert(input_type, result_type);
        }
        m
    };

    registry.get(input).copied()
}

/// Trait for computing arcsine of ROSY data types.
pub trait RosyASIN {
    type Output;
    fn rosy_asin(&self) -> anyhow::Result<Self::Output>;
}

/// ASIN for real numbers
impl RosyASIN for RE {
    type Output = RE;
    fn rosy_asin(&self) -> anyhow::Result<Self::Output> {
        Ok(self.asin())
    }
}

/// ASIN for vectors (elementwise)
impl RosyASIN for VE {
    type Output = VE;
    fn rosy_asin(&self) -> anyhow::Result<Self::Output> {
        Ok(self.iter().map(|x| x.asin()).collect())
    }
}

/// ASIN for DA (Taylor composition)
///
/// Uses Taylor series: asin(f₀ + δf) = asin(f₀) + Σ (d^n/dx^n asin(x)|_{x=f₀} / n!) * (δf)^n
/// asin'(x) = 1/sqrt(1-x²), higher derivatives computed numerically via recurrence.
impl RosyASIN for DA {
    type Output = DA;
    fn rosy_asin(&self) -> anyhow::Result<Self::Output> {
        da_asin(self)
    }
}

/// Compute arcsine of a DA object using Taylor series composition.
fn da_asin(da: &DA) -> anyhow::Result<DA> {
    use crate::rosy_lib::taylor::DACoefficient;

    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;

    let f0 = da.constant_part();

    // Create DA with constant part removed (δf = f - f₀)
    let mut da_prime = da.clone();
    da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);

    // Compute derivatives of asin at f0 numerically via recurrence.
    // d^0 asin(x) = asin(x)
    // d^1 asin(x) = (1 - x^2)^(-1/2)
    // d^n asin(x) for n >= 2: use numerical differentiation via finite differences
    // We use the recurrence: (1-x²) * f^(n+1)(x) + (2n-1)*x * f^(n)(x) + (n-1)^2 * f^(n-1)(x) = 0
    // => f^(n+1)(x) = -[(2n-1)*x * f^(n)(x) + (n-1)^2 * f^(n-1)(x)] / (1 - x^2)
    let mut derivs = vec![0.0f64; max_order + 1];
    derivs[0] = f0.asin();
    if max_order >= 1 {
        let denom = (1.0 - f0 * f0).sqrt();
        if denom.abs() < 1e-15 {
            return Err(anyhow::anyhow!("ASIN is not differentiable at x = ±1"));
        }
        derivs[1] = 1.0 / denom;
    }
    if max_order >= 2 {
        let denom = 1.0 - f0 * f0;
        if denom.abs() < 1e-15 {
            return Err(anyhow::anyhow!("ASIN is not differentiable at x = ±1"));
        }
        for n in 1..max_order {
            // f^(n+1)(x) = [(2n-1)*x*f^(n)(x) + (n-1)^2 * f^(n-1)(x)] / (x^2 - 1)
            // Rearranged from (1-x²)*f^(n+1) + (2n-1)*x*f^(n) + (n-1)^2 * f^(n-1) = 0
            let n_f = n as f64;
            derivs[n + 1] = ((2.0 * n_f - 1.0) * f0 * derivs[n] + (n_f - 1.0).powi(2) * derivs[n - 1]) / (1.0 - f0 * f0);
        }
    }

    // Precompute Taylor coefficients c_n = derivs[n] / n!
    let mut taylor_coeffs = Vec::with_capacity(max_order + 1);
    let mut factorial = 1.0;
    for n in 0..=max_order {
        if n > 0 { factorial *= n as f64; }
        taylor_coeffs.push(derivs[n] / factorial);
    }

    // Horner's evaluation
    let mut result = DA::from_coeff(taylor_coeffs[max_order]);
    for n in (0..max_order).rev() {
        result = (&result * &da_prime)?;
        result.add_constant_in_place(taylor_coeffs[n]);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rosy_lib::intrinsics::test_utils::test_intrinsic_output_match;

    #[test]
    fn test_rosy_cosy_asin_match() {
        test_intrinsic_output_match("asin");
    }
}
