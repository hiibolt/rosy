use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, CM, ST, LO, VE, DA, CD};

/// Type registry for TYPE intrinsic function.
///
/// Returns the COSY type code as RE:
/// - RE=1, CM=2, CD=3, ST=4, LO=5, VE=6, GR=7, DA=8
pub const TYPE_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "1.5"),
    IntrinsicTypeRule::new("CM", "RE", "CM(1.5&2.5)"),
    IntrinsicTypeRule::new("VE", "RE", "1.5&2.5&3.5"),
    IntrinsicTypeRule::new("DA", "RE", "DA(1)"),
];

/// Get the return type of TYPE for a given input type.
/// TYPE always returns RE regardless of input.
pub fn get_return_type(input: &RosyType) -> Option<RosyType> {
    let registry: HashMap<RosyType, RosyType> = {
        let mut m = HashMap::new();
        let all = vec![
            (RosyType::RE(), RosyType::RE()),
            (RosyType::CM(), RosyType::RE()),
            (RosyType::CD(), RosyType::RE()),
            (RosyType::ST(), RosyType::RE()),
            (RosyType::LO(), RosyType::RE()),
            (RosyType::VE(), RosyType::RE()),
            (RosyType::DA(), RosyType::RE()),
        ];
        for (input_type, result_type) in all {
            m.insert(input_type, result_type);
        }
        m
    };

    registry.get(input).copied()
}

/// Trait for returning the COSY type code of ROSY data types.
pub trait RosyTYPE {
    fn rosy_type(&self) -> anyhow::Result<RE>;
}

/// TYPE for real numbers - code 1
impl RosyTYPE for RE {
    fn rosy_type(&self) -> anyhow::Result<RE> {
        Ok(1.0)
    }
}

/// TYPE for complex numbers - code 2
impl RosyTYPE for CM {
    fn rosy_type(&self) -> anyhow::Result<RE> {
        Ok(2.0)
    }
}

/// TYPE for complex DA - code 3
impl RosyTYPE for CD {
    fn rosy_type(&self) -> anyhow::Result<RE> {
        Ok(3.0)
    }
}

/// TYPE for strings - code 4
impl RosyTYPE for ST {
    fn rosy_type(&self) -> anyhow::Result<RE> {
        Ok(4.0)
    }
}

/// TYPE for logical - code 5
impl RosyTYPE for LO {
    fn rosy_type(&self) -> anyhow::Result<RE> {
        Ok(5.0)
    }
}

/// TYPE for vectors - code 6
impl RosyTYPE for VE {
    fn rosy_type(&self) -> anyhow::Result<RE> {
        Ok(6.0)
    }
}

/// TYPE for DA - code 8
impl RosyTYPE for DA {
    fn rosy_type(&self) -> anyhow::Result<RE> {
        Ok(8.0)
    }
}
