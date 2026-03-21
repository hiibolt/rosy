use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, CM, VE, DA};

/// Type registry for SINH intrinsic function.
///
/// According to COSY INFINITY manual, SINH supports:
/// - RE -> RE
/// - CM -> CM (complex hyperbolic sine)
/// - VE -> VE (elementwise)
/// - DA -> DA (Taylor composition)
pub const SINH_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "1.5"),
    IntrinsicTypeRule::new("CM", "CM", "CM(1.5&2.5)"),
    IntrinsicTypeRule::new("VE", "VE", "1.5&2.5&3.5"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1)"),
];

/// Get the return type of SINH for a given input type.
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

/// Trait for computing hyperbolic sine of ROSY data types.
pub trait RosySINH {
    type Output;
    fn rosy_sinh(&self) -> anyhow::Result<Self::Output>;
}

/// SINH for real numbers
impl RosySINH for RE {
    type Output = RE;
    fn rosy_sinh(&self) -> anyhow::Result<Self::Output> {
        Ok(self.sinh())
    }
}

/// SINH for complex numbers
impl RosySINH for CM {
    type Output = CM;
    fn rosy_sinh(&self) -> anyhow::Result<Self::Output> {
        Ok(self.sinh())
    }
}

/// SINH for vectors (elementwise)
impl RosySINH for VE {
    type Output = VE;
    fn rosy_sinh(&self) -> anyhow::Result<Self::Output> {
        Ok(self.iter().map(|x| x.sinh()).collect())
    }
}

/// SINH for DA (Taylor composition)
impl RosySINH for DA {
    type Output = DA;
    fn rosy_sinh(&self) -> anyhow::Result<Self::Output> {
        da_sinh(self)
    }
}

/// Compute hyperbolic sine of a DA object using Horner's method.
///
/// c_n = [sinh_f0, cosh_f0][n%2] / n!
fn da_sinh(da: &DA) -> anyhow::Result<DA> {
    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;

    let f0 = da.constant_part();
    let sinh_f0 = f0.sinh();
    let cosh_f0 = f0.cosh();

    let mut da_prime = da.clone();
    da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);

    let cycle = [sinh_f0, cosh_f0];
    let mut taylor_coeffs = Vec::with_capacity(max_order + 1);
    let mut factorial = 1.0;
    for n in 0..=max_order {
        if n > 0 { factorial *= n as f64; }
        taylor_coeffs.push(cycle[n % 2] / factorial);
    }

    DA::horner_eval(&da_prime, &taylor_coeffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rosy_lib::intrinsics::test_utils::test_intrinsic_output_match;

    #[test]
    fn test_rosy_cosy_sinh_match() {
        test_intrinsic_output_match("sinh");
    }
}
