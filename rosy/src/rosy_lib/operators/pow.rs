//! Power/exponentiation operator for ROSY types.
//!
//! This module provides the `RosyPow` trait and implementations for all
//! supported type combinations. The compatibility rules are defined in the
//! `POW_REGISTRY` constant below.
//!
//! # Type Compatibility
//! 
//! According to COSY INFINITY manual:
//! - RE ^ RE -> RE
//! - VE ^ RE -> VE (component-wise)

use anyhow::Result;
use crate::rosy_lib::RosyType;
use crate::rosy_lib::{RE, VE};
use crate::rosy_lib::operators::{TypeRule, build_type_registry};

/// Type compatibility registry for power/exponentiation operator.
/// 
/// This is the single source of truth for what type combinations are allowed.
pub const POW_REGISTRY: &[TypeRule] = &[
    TypeRule::new("RE", "RE", "RE", "2", "3"),
    TypeRule::with_comment("VE", "RE", "VE", "1&2&3", "2", "Raise to Real power componentwise"),
];

pub fn get_return_type(lhs: &RosyType, rhs: &RosyType) -> Option<RosyType> {
    let registry = build_type_registry(POW_REGISTRY);
    registry.get(&(*lhs, *rhs)).copied()
}

pub trait RosyPow<Rhs = Self> {
    type Output;
    fn rosy_pow(self, rhs: Rhs) -> Result<Self::Output>;
}

// RE ^ RE
impl RosyPow<&RE> for &RE {
    type Output = RE;
    fn rosy_pow(self, rhs: &RE) -> Result<Self::Output> {
        Ok(self.powf(*rhs))
    }
}

// VE ^ RE (componentwise)
impl RosyPow<&RE> for &VE {
    type Output = VE;
    fn rosy_pow(self, rhs: &RE) -> Result<Self::Output> {
        Ok(self.iter().map(|x| x.powf(*rhs)).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rosy_lib::operators::test_utils::test_operator_output_match;

    #[test]
    fn test_rosy_cosy_pow_match() {
        test_operator_output_match("pow");
    }
}
