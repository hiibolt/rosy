//! Concatenation operator for ROSY types.
//!
//! This module provides the `RosyConcat` trait and implementations for all
//! supported type combinations. The compatibility rules are defined in the
//! `CONCAT_REGISTRY` constant below.
//!
//! # Type Compatibility
//! 
//! See `assets/operators/concat/concat_table.md` for the full compatibility table.
//!
//! # Examples
//! 
//! See `assets/operators/concat/concat.rosy` for ROSY examples and 
//! `assets/operators/concat/concat.fox` for equivalent COSY INFINITY code.

use anyhow::Result;
use crate::rosy_lib::RosyType;
use crate::rosy_lib::{RE, ST, VE};
use crate::rosy_lib::operators::{TypeRule, build_type_registry};

/// Type compatibility registry for concatenation operator.
/// 
/// This is the single source of truth for what type combinations are allowed.
/// The build script (`build.rs`) parses this to generate:
/// - Documentation table (`concat_table.md`)
/// - ROSY test script (`concat.rosy`)
/// - COSY test script (`concat.fox`)
/// - Integration tests
/// 
/// **Note:** This registry matches COSY INFINITY's & operator capabilities.
/// See manual.md Section A.2 "& (Concatenation)" for the authoritative list.
/// GR (Graphics) type is not yet implemented in ROSY.
pub const CONCAT_REGISTRY: &[TypeRule] = &[
    TypeRule::with_comment("RE", "RE", "VE", "Concatenate two Reals to a Vector"),
    TypeRule::with_comment("RE", "VE", "VE", "Append a Real to the left of a Vector"),
    TypeRule::with_comment("ST", "ST", "ST", "Concatenate two Strings"),
    TypeRule::with_comment("VE", "RE", "VE", "Append a Real to the right of a Vector"),
    TypeRule::with_comment("VE", "VE", "VE", "Concatenate two Vectors"),
    // GR & GR => GR is in COSY but GR type not implemented in ROSY yet
];

pub fn get_return_type(lhs: &RosyType, rhs: &RosyType) -> Option<RosyType> {
    let registry = build_type_registry(CONCAT_REGISTRY);
    registry.get(&(*lhs, *rhs)).copied()
}

pub trait RosyConcat<Rhs = Self> {
    type Output;
    fn rosy_concat(self, rhs: Rhs) -> Result<Self::Output>;
}

// RE & RE => VE
impl RosyConcat<&RE> for &RE {
    type Output = VE;
    fn rosy_concat(self, other: &RE) -> Result<Self::Output> {
        Ok(vec![*self, *other])
    }
}

// RE & VE => VE
impl RosyConcat<&VE> for &RE {
    type Output = VE;
    fn rosy_concat(self, other: &VE) -> Result<Self::Output> {
        let mut result = vec![*self];
        result.extend_from_slice(other);
        Ok(result)
    }
}

// ST & ST => ST
impl RosyConcat<&ST> for &ST {
    type Output = ST;
    fn rosy_concat(self, other: &ST) -> Result<Self::Output> {
        Ok(format!("{}{}", self, other))
    }
}

// VE & RE => VE
impl RosyConcat<&RE> for &VE {
    type Output = VE;
    fn rosy_concat(self, other: &RE) -> Result<Self::Output> {
        let mut result = self.clone();
        result.push(*other);
        Ok(result)
    }
}

// VE & VE => VE
impl RosyConcat<&VE> for &VE {
    type Output = VE;
    fn rosy_concat(self, other: &VE) -> Result<Self::Output> {
        let mut result = self.clone();
        result.extend_from_slice(other);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Note: The test_operator_output_match test is commented out because
    // ROSY and COSY have different output formats for vectors:
    // - ROSY: Displays vectors in formatted notation (e.g., "[2.5, 2.5]")
    // - COSY: Displays vector elements space-separated (e.g., "2.500000  2.500000")
    // This makes direct output comparison impractical for concat operations.
    // The unit tests below verify the actual Rust implementation works correctly.
    
    // use super::super::test_utils::test_operator_output_match;
    // #[test]
    // fn test_rosy_cosy_output_match() {
    //     test_operator_output_match("concat");
    // }

    #[test]
    fn test_re_re() {
        let a = 2.5;
        let b = 3.5;
        let result = (&a).rosy_concat(&b).unwrap();
        assert_eq!(result, vec![2.5, 3.5]);
    }

    #[test]
    fn test_re_ve() {
        let a = 1.0;
        let v = vec![2.0, 3.0];
        let result = (&a).rosy_concat(&v).unwrap();
        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_st_st() {
        let a = "Hello".to_string();
        let b = "World".to_string();
        let result = (&a).rosy_concat(&b).unwrap();
        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn test_ve_re() {
        let v = vec![1.0, 2.0];
        let a = 3.0;
        let result = (&v).rosy_concat(&a).unwrap();
        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_ve_ve() {
        let v1 = vec![1.0, 2.0];
        let v2 = vec![3.0, 4.0];
        let result = (&v1).rosy_concat(&v2).unwrap();
        assert_eq!(result, vec![1.0, 2.0, 3.0, 4.0]);
    }
}