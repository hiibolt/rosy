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

/// Compute hyperbolic cosine of a DA object using Horner's method.
///
/// c_n = [cosh_f0, sinh_f0][n%2] / n!
fn da_cosh(da: &DA) -> anyhow::Result<DA> {
    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;

    let f0 = da.constant_part();
    let sinh_f0 = f0.sinh();
    let cosh_f0 = f0.cosh();

    let mut da_prime = da.clone();
    da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);

    let cycle = [cosh_f0, sinh_f0];
    let mut taylor_coeffs = Vec::with_capacity(max_order + 1);
    let mut factorial = 1.0;
    for n in 0..=max_order {
        if n > 0 { factorial *= n as f64; }
        taylor_coeffs.push(cycle[n % 2] / factorial);
    }

    DA::horner_eval(&da_prime, &taylor_coeffs)
}

