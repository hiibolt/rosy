use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, CM, VE, DA};

/// Type registry for LOG intrinsic function.
///
/// According to COSY INFINITY manual, LOG supports:
/// - RE -> RE
/// - CM -> CM
/// - VE -> VE (elementwise)
/// - DA -> DA (Taylor composition)
///
/// Note: DA test value uses DA(1) + 1.0 (= 1 + x) instead of DA(1) (= x)
/// because LOG requires a non-zero constant part. DA(1) has constant part 0,
/// which would cause a domain error (ln(0) is undefined).
pub const LOG_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "1.5"),
    IntrinsicTypeRule::new("CM", "CM", "CM(1.5&2.5)"),
    IntrinsicTypeRule::new("VE", "VE", "1.5&2.5&3.5"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1) + 1.0"),
];

/// Get the return type of LOG for a given input type.
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

/// Trait for computing the natural logarithm of ROSY data types.
pub trait RosyLOG {
    type Output;
    fn rosy_log(&self) -> anyhow::Result<Self::Output>;
}

/// LOG for real numbers (uses f64::ln — the natural log)
impl RosyLOG for RE {
    type Output = RE;
    fn rosy_log(&self) -> anyhow::Result<Self::Output> {
        Ok(self.ln())
    }
}

/// LOG for complex numbers
impl RosyLOG for CM {
    type Output = CM;
    fn rosy_log(&self) -> anyhow::Result<Self::Output> {
        Ok(self.ln())
    }
}

/// LOG for vectors (elementwise natural log)
impl RosyLOG for VE {
    type Output = VE;
    fn rosy_log(&self) -> anyhow::Result<Self::Output> {
        Ok(self.iter().map(|x| x.ln()).collect())
    }
}

/// LOG for DA (Taylor composition).
///
/// Uses: ln(f) = ln(f₀) + sum_{n=1}^{N} (-1)^(n+1) / n * (δf / f₀)^n
/// where f₀ is the constant part and δf = f - f₀.
impl RosyLOG for DA {
    type Output = DA;
    fn rosy_log(&self) -> anyhow::Result<Self::Output> {
        da_log(self)
    }
}

fn da_log(da: &DA) -> anyhow::Result<DA> {
    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;

    let f0 = da.constant_part();
    anyhow::ensure!(f0 != 0.0, "LOG: constant part of DA argument must be non-zero");

    let ln_f0 = f0.ln();

    let mut da_prime = da.clone();
    da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);

    // u = δf / f₀
    let u = (&da_prime * DA::from_coeff(1.0 / f0))?;

    // ln(f) = ln(f₀) + u - u²/2 + u³/3 - ...
    // Precompute: c_0 = ln(f₀), c_n = (-1)^(n+1) / n
    let mut taylor_coeffs = Vec::with_capacity(max_order + 1);
    taylor_coeffs.push(ln_f0);
    for n in 1..=max_order {
        let sign = if n % 2 == 1 { 1.0 } else { -1.0 };
        taylor_coeffs.push(sign / (n as f64));
    }

    // Horner's on u
    let mut result = DA::from_coeff(taylor_coeffs[max_order]);
    for n in (0..max_order).rev() {
        result = (&result * &u)?;
        result.add_constant_in_place(taylor_coeffs[n]);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rosy_lib::intrinsics::test_utils::test_intrinsic_output_match;

    #[test]
    fn test_rosy_cosy_log_match() {
        test_intrinsic_output_match("log");
    }
}
