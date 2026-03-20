use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, VE, DA, CD};

/// Type registry for NORM intrinsic function.
///
/// According to COSY INFINITY manual, NORM supports:
/// - VE -> VE (elementwise abs)
/// - DA -> RE (max coefficient abs, i.e. max norm)
/// - CD -> RE (max coefficient abs of complex DA)
pub const NORM_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("VE", "VE", "1.5&2.5&3.5"),
    IntrinsicTypeRule::new("DA", "RE", "DA(1)"),
    IntrinsicTypeRule::new("CD", "RE", "CD(1)"),
];

/// Get the return type of NORM for a given input type.
pub fn get_return_type(input: &RosyType) -> Option<RosyType> {
    let registry: HashMap<RosyType, RosyType> = {
        let mut m = HashMap::new();
        let all = vec![
            (RosyType::VE(), RosyType::VE()),
            (RosyType::DA(), RosyType::RE()),
            (RosyType::CD(), RosyType::RE()),
        ];
        for (input_type, result_type) in all {
            m.insert(input_type, result_type);
        }
        m
    };

    registry.get(input).copied()
}

/// Trait for computing the norm of ROSY data types.
pub trait RosyNORM {
    type Output;
    fn rosy_norm(&self) -> anyhow::Result<Self::Output>;
}

/// NORM for vectors - elementwise abs
impl RosyNORM for VE {
    type Output = VE;
    fn rosy_norm(&self) -> anyhow::Result<Self::Output> {
        Ok(self.iter().map(|x| x.abs()).collect())
    }
}

/// NORM for DA - max coefficient abs (max norm)
impl RosyNORM for DA {
    type Output = RE;
    fn rosy_norm(&self) -> anyhow::Result<Self::Output> {
        Ok(self.coeffs_iter().map(|(_, c)| c.abs()).fold(0.0f64, f64::max))
    }
}

/// NORM for CD - max coefficient abs
impl RosyNORM for CD {
    type Output = RE;
    fn rosy_norm(&self) -> anyhow::Result<Self::Output> {
        use crate::rosy_lib::taylor::DACoefficient;
        Ok(self.coeffs_iter().map(|(_, c)| c.abs()).fold(0.0f64, f64::max))
    }
}
