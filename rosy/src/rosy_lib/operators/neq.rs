//! Not-equals operator for ROSY types.
//!
//! This module provides the `RosyNeq` trait and implementations for all
//! supported type combinations. The compatibility rules are defined in the
//! `NEQ_REGISTRY` constant below.
//!
//! # Type Compatibility
//! 
//! See `assets/operators/neq/neq_table.md` for the full compatibility table.
//!
//! # Examples
//! 
//! See `assets/operators/neq/neq.rosy` for ROSY examples and 
//! `assets/operators/neq/neq.fox` for equivalent COSY INFINITY code.

use anyhow::Result;
use crate::rosy_lib::RosyType;
use crate::rosy_lib::{RE, ST, LO};
use crate::rosy_lib::operators::{TypeRule, build_type_registry};

/// Type compatibility registry for not-equals operator.
/// 
/// This is the single source of truth for what type combinations are allowed.
/// The build script (`build.rs`) parses this to generate:
/// - Documentation table (`neq_table.md`)
/// - ROSY test script (`neq.rosy`)
/// - COSY test script (`neq.fox`)
/// - Integration tests
pub const NEQ_REGISTRY: &[TypeRule] = &[
    TypeRule::with_comment("RE", "RE", "LO", "3.14159", "2.71828", "Not-equals with epsilon tolerance"),
    TypeRule::with_comment("ST", "ST", "LO", "'hello'", "'world'", "String not-equals"),
    TypeRule::with_comment("LO", "LO", "LO", "TRUE", "FALSE", "Logical not-equals"),
];

pub fn get_return_type(lhs: &RosyType, rhs: &RosyType) -> Option<RosyType> {
    let registry = build_type_registry(NEQ_REGISTRY);
    registry.get(&(*lhs, *rhs)).copied()
}

pub trait RosyNeq<Rhs = Self> {
    type Output;
    fn rosy_neq(self, rhs: Rhs) -> Result<Self::Output>;
}

// RE # RE (with epsilon tolerance)
impl RosyNeq<&RE> for &RE {
    type Output = LO;
    fn rosy_neq(self, rhs: &RE) -> Result<Self::Output> {
        Ok((self - rhs).abs() >= f64::EPSILON)
    }
}

// ST # ST (exact string inequality)
impl RosyNeq<&ST> for &ST {
    type Output = LO;
    fn rosy_neq(self, rhs: &ST) -> Result<Self::Output> {
        Ok(self != rhs)
    }
}

// LO # LO (logical inequality)
impl RosyNeq<&LO> for &LO {
    type Output = LO;
    fn rosy_neq(self, rhs: &LO) -> Result<Self::Output> {
        Ok(self != rhs)
    }
}
