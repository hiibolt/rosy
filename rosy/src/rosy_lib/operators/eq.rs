//! Equality operator for ROSY types.
//!
//! This module provides the `RosyEq` trait and implementations for all
//! supported type combinations. The compatibility rules are defined in the
//! `EQ_REGISTRY` constant below.
//!
//! # Type Compatibility
//! 
//! See `assets/operators/eq/eq_table.md` for the full compatibility table.
//!
//! # Examples
//! 
//! See `assets/operators/eq/eq.rosy` for ROSY examples and 
//! `assets/operators/eq/eq.fox` for equivalent COSY INFINITY code.

use anyhow::Result;
use crate::rosy_lib::RosyType;
use crate::rosy_lib::{RE, ST, LO};
use crate::rosy_lib::operators::{TypeRule, build_type_registry};

/// Type compatibility registry for equality operator.
/// 
/// This is the single source of truth for what type combinations are allowed.
/// The build script (`build.rs`) parses this to generate:
/// - Documentation table (`eq_table.md`)
/// - ROSY test script (`eq.rosy`)
/// - COSY test script (`eq.fox`)
/// - Integration tests
pub const EQ_REGISTRY: &[TypeRule] = &[
    TypeRule::with_comment("RE", "RE", "LO", "3.14159", "3.14159", "Equality with epsilon tolerance"),
    TypeRule::with_comment("ST", "ST", "LO", "'hello'", "'hello'", "String equality"),
    TypeRule::with_comment("LO", "LO", "LO", "TRUE", "TRUE", "Logical equality"),
];

pub fn get_return_type(lhs: &RosyType, rhs: &RosyType) -> Option<RosyType> {
    let registry = build_type_registry(EQ_REGISTRY);
    registry.get(&(*lhs, *rhs)).copied()
}

pub trait RosyEq<Rhs = Self> {
    type Output;
    fn rosy_eq(self, rhs: Rhs) -> Result<Self::Output>;
}

// RE = RE (with epsilon tolerance)
impl RosyEq<&RE> for &RE {
    type Output = LO;
    fn rosy_eq(self, rhs: &RE) -> Result<Self::Output> {
        Ok((self - rhs).abs() < f64::EPSILON)
    }
}

// ST = ST (exact string equality)
impl RosyEq<&ST> for &ST {
    type Output = LO;
    fn rosy_eq(self, rhs: &ST) -> Result<Self::Output> {
        Ok(self == rhs)
    }
}

// LO = LO (logical equality)
impl RosyEq<&LO> for &LO {
    type Output = LO;
    fn rosy_eq(self, rhs: &LO) -> Result<Self::Output> {
        Ok(self == rhs)
    }
}
