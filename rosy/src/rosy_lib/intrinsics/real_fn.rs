use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, CM, DA};

/// Type registry for REAL intrinsic function.
///
/// According to COSY INFINITY manual, REAL supports:
/// - RE -> RE (identity)
/// - CM -> RE (real part of complex)
/// - DA -> DA (identity)
pub const REAL_FN_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "1.5"),
    IntrinsicTypeRule::new("CM", "RE", "CM(1.5&2.5)"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1)"),
];

/// Get the return type of REAL for a given input type.
pub fn get_return_type(input: &RosyType) -> Option<RosyType> {
    let registry: HashMap<RosyType, RosyType> = {
        let mut m = HashMap::new();
        let all = vec![
            (RosyType::RE(), RosyType::RE()),
            (RosyType::CM(), RosyType::RE()),
            (RosyType::DA(), RosyType::DA()),
        ];
        for (input_type, result_type) in all {
            m.insert(input_type, result_type);
        }
        m
    };

    registry.get(input).copied()
}

/// Trait for computing the real part of ROSY data types.
pub trait RosyREAL {
    type Output;
    fn rosy_real(&self) -> anyhow::Result<Self::Output>;
}

/// REAL for real numbers - identity
impl RosyREAL for RE {
    type Output = RE;
    fn rosy_real(&self) -> anyhow::Result<Self::Output> {
        Ok(*self)
    }
}

/// REAL for complex numbers - real part
impl RosyREAL for CM {
    type Output = RE;
    fn rosy_real(&self) -> anyhow::Result<Self::Output> {
        Ok(self.re)
    }
}

/// REAL for DA - identity
impl RosyREAL for DA {
    type Output = DA;
    fn rosy_real(&self) -> anyhow::Result<Self::Output> {
        Ok(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rosy_lib::intrinsics::test_utils::test_intrinsic_output_match;

    #[test]
    fn test_rosy_cosy_real_fn_match() {
        test_intrinsic_output_match("real_fn");
    }
}
