use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, CM, VE, DA};

/// Type registry for COSH intrinsic function.
///
/// According to COSY INFINITY manual, COSH supports:
/// - RE -> RE
/// - CM -> CM (complex hyperbolic cosine)
/// - VE -> VE (elementwise)
/// - DA -> DA (Taylor composition)
pub const COSH_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "1.5"),
    IntrinsicTypeRule::new("CM", "CM", "CM(1.5&2.5)"),
    IntrinsicTypeRule::new("VE", "VE", "1.5&2.5&3.5"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1)"),
];

/// Get the return type of COSH for a given input type.
pub fn get_return_type(input: &RosyType) -> Option<RosyType> {
    let registry: HashMap<RosyType, RosyType> = {
        let mut m = HashMap::new();
        let all = vec![
            (RosyType::RE(), RosyType::RE()),
            (RosyType::CM(), RosyType::CM()),
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

/// Trait for computing hyperbolic cosine of ROSY data types.
pub trait RosyCOSH {
    type Output;
    fn rosy_cosh(&self) -> anyhow::Result<Self::Output>;
}

/// COSH for real numbers
impl RosyCOSH for RE {
    type Output = RE;
    fn rosy_cosh(&self) -> anyhow::Result<Self::Output> {
        Ok(self.cosh())
    }
}

/// COSH for complex numbers
impl RosyCOSH for CM {
    type Output = CM;
    fn rosy_cosh(&self) -> anyhow::Result<Self::Output> {
        Ok(self.cosh())
    }
}

/// COSH for vectors (elementwise)
impl RosyCOSH for VE {
    type Output = VE;
    fn rosy_cosh(&self) -> anyhow::Result<Self::Output> {
        Ok(self.iter().map(|x| x.cosh()).collect())
    }
}

/// COSH for DA (Taylor composition)
impl RosyCOSH for DA {
    type Output = DA;
    fn rosy_cosh(&self) -> anyhow::Result<Self::Output> {
        da_cosh(self)
    }
}

/// Compute hyperbolic cosine of a DA object using Taylor series composition.
///
/// cosh(f₀ + δf) = cosh(f₀) + sinh(f₀)·δf + cosh(f₀)·(δf)²/2! + sinh(f₀)·(δf)³/3! + ...
/// Coefficients cycle with period 2 (always positive): [sinh_f0, cosh_f0, sinh_f0, cosh_f0, ...]
fn da_cosh(da: &DA) -> anyhow::Result<DA> {
    use crate::rosy_lib::taylor::DACoefficient;

    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;

    let f0 = da.constant_part();
    let sinh_f0 = f0.sinh();
    let cosh_f0 = f0.cosh();

    // Create DA with constant part removed (δf = f - f₀)
    let mut da_prime = da.clone();
    da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);

    // Build result using Taylor composition
    // Coefficients for n-th derivative of cosh at f₀: [sinh_f0, cosh_f0, sinh_f0, cosh_f0, ...]
    let mut result = DA::from_coeff(cosh_f0);
    let mut term = da_prime.clone();
    let mut factorial = 1.0;

    let coeffs = [sinh_f0, cosh_f0];

    for n in 1..=max_order {
        factorial *= n as f64;
        let coeff_index = (n - 1) % 2;
        let coefficient = coeffs[coeff_index];

        let scaled_term = (&term * DA::from_coeff(coefficient / factorial))?;
        result = (&result + &scaled_term)?;

        if n < max_order {
            term = (&term * &da_prime)?;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rosy_lib::intrinsics::test_utils::test_intrinsic_output_match;

    #[test]
    fn test_rosy_cosy_cosh_match() {
        test_intrinsic_output_match("cosh");
    }
}
