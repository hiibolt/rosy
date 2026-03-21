use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, VE, DA};

/// Type registry for ACOS intrinsic function.
///
/// According to COSY INFINITY manual, ACOS supports:
/// - RE -> RE
/// - VE -> VE (elementwise)
/// - DA -> DA (Taylor composition)
///
/// Note: CM is NOT supported for ACOS in COSY.
pub const ACOS_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "0.5"),
    IntrinsicTypeRule::new("VE", "VE", "0.1&0.2&0.3"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1)"),
];

/// Get the return type of ACOS for a given input type.
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

/// Trait for computing arccosine of ROSY data types.
pub trait RosyACOS {
    type Output;
    fn rosy_acos(&self) -> anyhow::Result<Self::Output>;
}

/// ACOS for real numbers
impl RosyACOS for RE {
    type Output = RE;
    fn rosy_acos(&self) -> anyhow::Result<Self::Output> {
        Ok(self.acos())
    }
}

/// ACOS for vectors (elementwise)
impl RosyACOS for VE {
    type Output = VE;
    fn rosy_acos(&self) -> anyhow::Result<Self::Output> {
        Ok(self.iter().map(|x| x.acos()).collect())
    }
}

/// ACOS for DA (Taylor composition)
///
/// acos(x) = pi/2 - asin(x), so derivatives are the negatives of asin derivatives
/// (except the constant term).
impl RosyACOS for DA {
    type Output = DA;
    fn rosy_acos(&self) -> anyhow::Result<Self::Output> {
        da_acos(self)
    }
}

/// Compute arccosine of a DA object using Taylor series composition.
///
/// Uses the identity acos(x) = pi/2 - asin(x), so derivatives of acos are
/// the negative of the derivatives of asin (for n >= 1).
fn da_acos(da: &DA) -> anyhow::Result<DA> {
    use crate::rosy_lib::taylor::DACoefficient;

    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;

    let f0 = da.constant_part();

    // Create DA with constant part removed (δf = f - f₀)
    let mut da_prime = da.clone();
    da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);

    // Compute derivatives of acos at f0.
    // acos'(x) = -1/sqrt(1-x^2), and higher derivatives use the same recurrence as asin
    // but negated: since acos(x) = pi/2 - asin(x), d^n acos / dx^n = -d^n asin / dx^n for n >= 1.
    let mut derivs = vec![0.0f64; max_order + 1];
    derivs[0] = f0.acos();
    if max_order >= 1 {
        let denom = (1.0 - f0 * f0).sqrt();
        if denom.abs() < 1e-15 {
            return Err(anyhow::anyhow!("ACOS is not differentiable at x = ±1"));
        }
        derivs[1] = -1.0 / denom;
    }
    if max_order >= 2 {
        let denom = 1.0 - f0 * f0;
        if denom.abs() < 1e-15 {
            return Err(anyhow::anyhow!("ACOS is not differentiable at x = ±1"));
        }
        for n in 1..max_order {
            // Same recurrence as asin but negated: acos satisfies (1-x²)*f'' + (2n-1)*x*f' + (n-1)^2 * f_{n-1} = 0
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
    fn test_rosy_cosy_acos_match() {
        test_intrinsic_output_match("acos");
    }
}
