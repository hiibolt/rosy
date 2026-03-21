use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, VE, DA};

/// Type registry for ATAN intrinsic function.
///
/// According to COSY INFINITY manual, ATAN supports:
/// - RE -> RE
/// - VE -> VE (elementwise)
/// - DA -> DA (Taylor composition)
///
/// Note: CM is NOT supported for ATAN in COSY.
pub const ATAN_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "1.5"),
    IntrinsicTypeRule::new("VE", "VE", "1.5&2.5&3.5"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1)"),
];

/// Get the return type of ATAN for a given input type.
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

/// Trait for computing arctangent of ROSY data types.
pub trait RosyATAN {
    type Output;
    fn rosy_atan(&self) -> anyhow::Result<Self::Output>;
}

/// ATAN for real numbers
impl RosyATAN for RE {
    type Output = RE;
    fn rosy_atan(&self) -> anyhow::Result<Self::Output> {
        Ok(self.atan())
    }
}

/// ATAN for vectors (elementwise)
impl RosyATAN for VE {
    type Output = VE;
    fn rosy_atan(&self) -> anyhow::Result<Self::Output> {
        Ok(self.iter().map(|x| x.atan()).collect())
    }
}

/// ATAN for DA (Taylor composition)
///
/// atan'(x) = 1/(1+x²), higher derivatives computed via recurrence.
impl RosyATAN for DA {
    type Output = DA;
    fn rosy_atan(&self) -> anyhow::Result<Self::Output> {
        da_atan(self)
    }
}

/// Compute arctangent of a DA object using Taylor series composition.
///
/// Uses the recurrence for atan derived from (1+x²)*f'(x) = 1:
/// Differentiating n times: (1+x²)*f^(n+1) + 2*n*x*f^(n) + n*(n-1)*f^(n-1) = 0
/// => f^(n+1) = -[2*n*x*f^(n) + n*(n-1)*f^(n-1)] / (1+x²)
fn da_atan(da: &DA) -> anyhow::Result<DA> {
    use crate::rosy_lib::taylor::DACoefficient;

    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;

    let f0 = da.constant_part();

    // Create DA with constant part removed (δf = f - f₀)
    let mut da_prime = da.clone();
    da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);

    // Compute derivatives of atan at f0.
    // atan'(x) = 1/(1+x²)
    // Recurrence: (1+x²)*f^(n+1) + 2*n*x*f^(n) + n*(n-1)*f^(n-1) = 0
    let mut derivs = vec![0.0f64; max_order + 1];
    derivs[0] = f0.atan();
    if max_order >= 1 {
        derivs[1] = 1.0 / (1.0 + f0 * f0);
    }
    if max_order >= 2 {
        let denom = 1.0 + f0 * f0;
        for n in 1..max_order {
            let n_f = n as f64;
            derivs[n + 1] = -(2.0 * n_f * f0 * derivs[n] + n_f * (n_f - 1.0) * derivs[n - 1]) / denom;
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
    fn test_rosy_cosy_atan_match() {
        test_intrinsic_output_match("atan");
    }
}
