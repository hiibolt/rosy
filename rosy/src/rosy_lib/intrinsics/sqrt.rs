use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, CM, VE, DA};

/// Type registry for SQRT intrinsic function.
///
/// According to COSY INFINITY manual, SQRT supports:
/// - RE -> RE
/// - CM -> CM
/// - VE -> VE (elementwise)
/// - DA -> DA (Taylor composition)
///
/// Note: DA test value uses EXP(DA(1)) to ensure a positive constant part,
/// which is required for the binomial series expansion of sqrt.
pub const SQRT_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "4.0"),
    IntrinsicTypeRule::new("CM", "CM", "CM(3.0&4.0)"),
    IntrinsicTypeRule::new("VE", "VE", "1.0&4.0&9.0"),
    IntrinsicTypeRule::new("DA", "DA", "EXP(DA(1))"),
];

/// Get the return type of SQRT for a given input type.
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

/// Trait for computing the square root of ROSY data types.
pub trait RosySQRT {
    type Output;
    fn rosy_sqrt(&self) -> anyhow::Result<Self::Output>;
}

/// SQRT for real numbers
impl RosySQRT for RE {
    type Output = RE;
    fn rosy_sqrt(&self) -> anyhow::Result<Self::Output> {
        Ok(self.sqrt())
    }
}

/// SQRT for complex numbers
impl RosySQRT for CM {
    type Output = CM;
    fn rosy_sqrt(&self) -> anyhow::Result<Self::Output> {
        Ok(self.sqrt())
    }
}

/// SQRT for vectors (elementwise)
impl RosySQRT for VE {
    type Output = VE;
    fn rosy_sqrt(&self) -> anyhow::Result<Self::Output> {
        Ok(self.iter().map(|x| x.sqrt()).collect())
    }
}

/// SQRT for DA (Taylor composition via binomial series)
impl RosySQRT for DA {
    type Output = DA;
    fn rosy_sqrt(&self) -> anyhow::Result<Self::Output> {
        da_sqrt(self)
    }
}

/// Compute square root of a DA object using binomial series expansion.
///
/// Uses: sqrt(f) = sqrt(f0) * sqrt(1 + u)  where u = (f - f0) / f0
/// sqrt(1 + u) = sum_{n=0}^{N} C(1/2, n) * u^n
/// where C(1/2, n) = (1/2)(1/2-1)...(1/2-n+1) / n!
///
/// Requires: f0 = constant part of the DA > 0 (sqrt is not analytic at 0).
fn da_sqrt(da: &DA) -> anyhow::Result<DA> {
    use crate::rosy_lib::taylor::DACoefficient;

    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;

    let f0 = da.constant_part();
    anyhow::ensure!(f0 > 0.0, "SQRT: constant part of DA must be positive, got {}", f0);

    let sqrt_f0 = f0.sqrt();

    // Create delta = (f - f0) / f0
    let mut da_prime = da.clone();
    da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);
    let da_delta = (&da_prime * DA::from_coeff(1.0 / f0))?;

    // Precompute binomial coefficients C(1/2, n)
    let mut taylor_coeffs = Vec::with_capacity(max_order + 1);
    taylor_coeffs.push(1.0); // C(1/2, 0) = 1
    let mut binom_coeff = 0.5_f64;
    for n in 1..=max_order {
        taylor_coeffs.push(binom_coeff);
        binom_coeff *= (0.5 - n as f64) / (n as f64 + 1.0);
    }

    // Horner's evaluation of (1 + u)^(1/2)
    let mut result = DA::horner_eval(&da_delta, &taylor_coeffs)?;

    // Multiply by sqrt(f0)
    result = (&result * DA::from_coeff(sqrt_f0))?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rosy_lib::intrinsics::test_utils::test_intrinsic_output_match;

    #[test]
    fn test_rosy_cosy_sqrt_match() {
        test_intrinsic_output_match("sqrt");
    }
}
