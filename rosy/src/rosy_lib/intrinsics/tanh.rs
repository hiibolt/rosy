use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, VE, DA};

/// Type registry for TANH intrinsic function.
///
/// According to COSY INFINITY manual, TANH supports:
/// - RE -> RE
/// - VE -> VE (elementwise)
/// - DA -> DA (Taylor composition)
///
/// Note: CM is NOT supported for TANH in COSY.
pub const TANH_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "1.5"),
    IntrinsicTypeRule::new("VE", "VE", "1.5&2.5&3.5"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1)"),
];

/// Get the return type of TANH for a given input type.
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

/// Trait for computing hyperbolic tangent of ROSY data types.
pub trait RosyTANH {
    type Output;
    fn rosy_tanh(&self) -> anyhow::Result<Self::Output>;
}

/// TANH for real numbers
impl RosyTANH for RE {
    type Output = RE;
    fn rosy_tanh(&self) -> anyhow::Result<Self::Output> {
        Ok(self.tanh())
    }
}

/// TANH for vectors (elementwise)
impl RosyTANH for VE {
    type Output = VE;
    fn rosy_tanh(&self) -> anyhow::Result<Self::Output> {
        Ok(self.iter().map(|x| x.tanh()).collect())
    }
}

/// TANH for DA (Taylor composition)
/// Uses: tanh(f) = sinh(f) / cosh(f)
impl RosyTANH for DA {
    type Output = DA;
    fn rosy_tanh(&self) -> anyhow::Result<Self::Output> {
        use crate::rosy_lib::intrinsics::sinh::RosySINH;
        use crate::rosy_lib::intrinsics::cosh::RosyCOSH;

        let sinh_f = self.rosy_sinh()?;
        let cosh_f = self.rosy_cosh()?;

        (&sinh_f / &cosh_f).map_err(|e| e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rosy_lib::intrinsics::test_utils::test_intrinsic_output_match;

    #[test]
    fn test_rosy_cosy_tanh_match() {
        test_intrinsic_output_match("tanh");
    }
}
