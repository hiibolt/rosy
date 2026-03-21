use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, CM, VE, DA, CD};

/// Type registry for COS intrinsic function.
///
/// According to COSY INFINITY manual, COS supports:
/// - RE -> RE
/// - CM -> CM (complex cos)
/// - VE -> VE (elementwise)
/// - DA -> DA (Taylor composition)
pub const COS_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "1.5"),
    IntrinsicTypeRule::new("CM", "CM", "CM(1.5&2.5)"),
    IntrinsicTypeRule::new("VE", "VE", "1.5&2.5&3.5"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1)"),
    IntrinsicTypeRule::new("CD", "CD", "CD(1)"),
];

/// Get the return type of COS for a given input type.
pub fn get_return_type(input: &RosyType) -> Option<RosyType> {
    let registry: HashMap<RosyType, RosyType> = {
        let mut m = HashMap::new();
        let all = vec![
            (RosyType::RE(), RosyType::RE()),
            (RosyType::CM(), RosyType::CM()),
            (RosyType::VE(), RosyType::VE()),
            (RosyType::DA(), RosyType::DA()),
            (RosyType::CD(), RosyType::CD()),
        ];
        for (input_type, result_type) in all {
            m.insert(input_type, result_type);
        }
        m
    };

    registry.get(input).copied()
}

/// Trait for computing cosine of ROSY data types.
pub trait RosyCOS {
    type Output;
    fn rosy_cos(&self) -> anyhow::Result<Self::Output>;
}

/// COS for real numbers
impl RosyCOS for RE {
    type Output = RE;
    fn rosy_cos(&self) -> anyhow::Result<Self::Output> {
        Ok(self.cos())
    }
}

/// COS for complex numbers
impl RosyCOS for CM {
    type Output = CM;
    fn rosy_cos(&self) -> anyhow::Result<Self::Output> {
        Ok(self.cos())
    }
}

/// COS for vectors (elementwise)
impl RosyCOS for VE {
    type Output = VE;
    fn rosy_cos(&self) -> anyhow::Result<Self::Output> {
        Ok(self.iter().map(|x| x.cos()).collect())
    }
}

/// COS for DA (Taylor composition)
impl RosyCOS for DA {
    type Output = DA;
    fn rosy_cos(&self) -> anyhow::Result<Self::Output> {
        da_cos(self)
    }
}

/// COS for CD (complex Taylor composition)
impl RosyCOS for CD {
    type Output = CD;
    fn rosy_cos(&self) -> anyhow::Result<Self::Output> {
        cd_cos(self)
    }
}

/// Compute cosine of a DA object using Taylor series composition.
///
/// Evaluates P(δf) = c₀ + δf·(c₁ + δf·(c₂ + ...)) using Horner's method
/// where c_n = d^n(cos)(f₀)/n!, cycle: [cos, -sin, -cos, sin]
fn da_cos(da: &DA) -> anyhow::Result<DA> {
    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;

    let f0 = da.constant_part();
    let sin_f0 = f0.sin();
    let cos_f0 = f0.cos();

    let mut da_prime = da.clone();
    da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);

    let cycle = [cos_f0, -sin_f0, -cos_f0, sin_f0];
    let mut taylor_coeffs = Vec::with_capacity(max_order + 1);
    let mut factorial = 1.0;
    for n in 0..=max_order {
        if n > 0 { factorial *= n as f64; }
        taylor_coeffs.push(cycle[n % 4] / factorial);
    }

    DA::horner_eval(&da_prime, &taylor_coeffs)
}

/// Compute cosine of a CD object using Horner's method.
fn cd_cos(cd: &CD) -> anyhow::Result<CD> {
    use crate::rosy_lib::taylor::DACoefficient;
    use num_complex::Complex64;

    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;

    let f0 = cd.constant_part();
    let sin_f0 = f0.sin();
    let cos_f0 = f0.cos();

    let mut cd_prime = cd.clone();
    cd_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), Complex64::zero());

    let cycle = [cos_f0, -sin_f0, -cos_f0, sin_f0];
    let mut taylor_coeffs = Vec::with_capacity(max_order + 1);
    let mut factorial = 1.0;
    for n in 0..=max_order {
        if n > 0 { factorial *= n as f64; }
        taylor_coeffs.push(cycle[n % 4] / factorial);
    }

    CD::horner_eval(&cd_prime, &taylor_coeffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rosy_lib::intrinsics::test_utils::test_intrinsic_output_match;

    #[test]
    fn test_rosy_cosy_cos_match() {
        test_intrinsic_output_match("cos");
    }
}
