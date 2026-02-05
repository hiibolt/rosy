//! Less-than operator for ROSY types.
//!
//! This module provides the `RosyLt` trait and implementations for all
//! supported type combinations. The compatibility rules are defined in the
//! `LT_REGISTRY` constant below.

use anyhow::Result;
use crate::rosy_lib::RosyType;
use crate::rosy_lib::{RE, ST, LO};
use crate::rosy_lib::operators::{TypeRule, build_type_registry};

/// Type compatibility registry for less-than operator.
pub const LT_REGISTRY: &[TypeRule] = &[
    TypeRule::with_comment("RE", "RE", "LO", "1.0", "2.0", "Numeric less-than"),
    TypeRule::with_comment("ST", "ST", "LO", "'apple'", "'banana'", "Lexicographic ordering"),
];

pub fn get_return_type(lhs: &RosyType, rhs: &RosyType) -> Option<RosyType> {
    let registry = build_type_registry(LT_REGISTRY);
    registry.get(&(*lhs, *rhs)).copied()
}

pub trait RosyLt<Rhs = Self> {
    type Output;
    fn rosy_lt(self, rhs: Rhs) -> Result<Self::Output>;
}

// RE < RE
impl RosyLt<&RE> for &RE {
    type Output = LO;
    fn rosy_lt(self, rhs: &RE) -> Result<Self::Output> {
        Ok(self < rhs)
    }
}

// ST < ST (lexicographic ordering)
impl RosyLt<&ST> for &ST {
    type Output = LO;
    fn rosy_lt(self, rhs: &ST) -> Result<Self::Output> {
        Ok(self < rhs)
    }
}
