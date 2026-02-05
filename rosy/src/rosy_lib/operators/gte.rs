//! Greater-than-or-equal operator for ROSY types.
//!
//! This is a ROSY extension not present in COSY INFINITY.
//!
//! This module provides the `RosyGte` trait and implementations for all
//! supported type combinations. The compatibility rules are defined in the
//! `GTE_REGISTRY` constant below.

use anyhow::Result;
use crate::rosy_lib::RosyType;
use crate::rosy_lib::{RE, ST, LO};
use crate::rosy_lib::operators::{TypeRule, build_type_registry};

/// Type compatibility registry for greater-than-or-equal operator.
pub const GTE_REGISTRY: &[TypeRule] = &[
    TypeRule::with_comment("RE", "RE", "LO", "2.0", "2.0", "Numeric greater-than-or-equal"),
    TypeRule::with_comment("ST", "ST", "LO", "'banana'", "'banana'", "Lexicographic ordering"),
];

pub fn get_return_type(lhs: &RosyType, rhs: &RosyType) -> Option<RosyType> {
    let registry = build_type_registry(GTE_REGISTRY);
    registry.get(&(*lhs, *rhs)).copied()
}

pub trait RosyGte<Rhs = Self> {
    type Output;
    fn rosy_gte(self, rhs: Rhs) -> Result<Self::Output>;
}

// RE >= RE
impl RosyGte<&RE> for &RE {
    type Output = LO;
    fn rosy_gte(self, rhs: &RE) -> Result<Self::Output> {
        Ok(self >= rhs)
    }
}

// ST >= ST (lexicographic ordering)
impl RosyGte<&ST> for &ST {
    type Output = LO;
    fn rosy_gte(self, rhs: &ST) -> Result<Self::Output> {
        Ok(self >= rhs)
    }
}
