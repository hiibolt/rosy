//! Addition operator for ROSY types.
//!
//! This module provides the `RosyAdd` trait and implementations for all
//! supported type combinations. The compatibility rules are defined in the
//! `ADD_REGISTRY` constant below.
//!
//! # Type Compatibility
#![doc = include_str!("add_table.md")]
//!
//! # Examples
//! ## ROSY
#![doc = "```ignore"]
#![doc = include_str!("add.rosy")]
#![doc = "```"]
//! ## Equivalent COSY INFINITY
#![doc = "```ignore"]
#![doc = include_str!("add.fox")]
#![doc = "```"]

use std::collections::HashMap;
use anyhow::Result;
use crate::rosy_lib::RosyType;
use crate::rosy_lib::{RE, CM, VE, DA, CD, LO};
use crate::rosy_lib::operators::registry::TypeRule;

/// Type compatibility registry for addition operator.
/// 
/// This is the single source of truth for what type combinations are allowed.
/// The build script (`build.rs`) parses this to generate:
/// - Documentation table (`add_table.md`)
/// - ROSY test script (`add.rosy`)
/// - COSY test script (`add.fox`)
/// - Integration tests
pub const ADD_REGISTRY: &[TypeRule] = &[
    TypeRule::new("RE", "RE", "RE"),
    TypeRule::new("RE", "CM", "CM"),
    TypeRule::with_comment("RE", "VE", "VE", "Add Real componentwise"),
    TypeRule::new("RE", "DA", "DA"),
    TypeRule::new("RE", "CD", "CD"),
    TypeRule::with_comment("LO", "LO", "LO", "Logical OR"),
    TypeRule::new("CM", "RE", "CM"),
    TypeRule::new("CM", "CM", "CM"),
    TypeRule::new("CM", "DA", "CD"),
    TypeRule::new("CM", "CD", "CD"),
    TypeRule::with_comment("VE", "RE", "VE", "Add Real componentwise"),
    TypeRule::with_comment("VE", "VE", "VE", "Add componentwise"),
    TypeRule::new("DA", "RE", "DA"),
    TypeRule::new("DA", "CM", "CD"),
    TypeRule::new("DA", "DA", "DA"),
    TypeRule::new("DA", "CD", "CD"),
    TypeRule::new("CD", "RE", "CD"),
    TypeRule::new("CD", "CM", "CD"),
    TypeRule::new("CD", "DA", "CD"),
    TypeRule::new("CD", "CD", "CD"),
];

/// Helper function to convert type name string to RosyType
fn type_from_str(s: &str) -> RosyType {
    match s {
        "RE" => RosyType::RE(),
        "ST" => RosyType::ST(),
        "LO" => RosyType::LO(),
        "CM" => RosyType::CM(),
        "VE" => RosyType::VE(),
        "DA" => RosyType::DA(),
        "CD" => RosyType::CD(),
        _ => panic!("Unknown type: {}", s),
    }
}

pub fn get_return_type ( lhs: &RosyType, rhs: &RosyType ) -> Option<RosyType> {
    let registry: HashMap<(RosyType, RosyType), RosyType> = {
        let mut m = HashMap::new();
        // Dynamically build from ADD_REGISTRY - single source of truth
        for rule in ADD_REGISTRY {
            let left = type_from_str(rule.lhs);
            let right = type_from_str(rule.rhs);
            let result = type_from_str(rule.result);
            m.insert((left, right), result);
        }
        m
    };

    registry.get(&(*lhs, *rhs)).copied()
}

pub trait RosyAdd<Rhs = Self> {
    type Output;
    fn rosy_add(self, rhs: Rhs) -> Result<Self::Output>;
}
// RE + RE
impl RosyAdd<&RE> for &RE {
    type Output = RE;
    fn rosy_add(self, rhs: &RE) -> Result<Self::Output> {
        Ok(self + rhs)
    }
}
// RE + CM
impl RosyAdd<&CM> for &RE {
    type Output = CM;
    fn rosy_add(self, other: &CM) -> Result<Self::Output> {
        Ok((self + other.0, other.1))
    }
}
// RE + VE
impl RosyAdd<&VE> for &RE {
    type Output = VE;
    fn rosy_add(self, other: &VE) -> Result<Self::Output> {
        Ok(other.iter().map(|x| x + self).collect())
    }
}

// RE + DA
impl RosyAdd<&DA> for &RE {
    type Output = DA;
    fn rosy_add(self, other: &DA) -> Result<Self::Output> {
        other + *self
    }
}

// CM + RE
impl RosyAdd<&RE> for &CM {
    type Output = CM;
    fn rosy_add(self, other: &RE) -> Result<Self::Output> {
        Ok((self.0 + other, self.1))
    }
}
// CM + CM
impl RosyAdd<&CM> for &CM {
    type Output = CM;
    fn rosy_add(self, other: &CM) -> Result<Self::Output> {
        Ok((self.0 + other.0, self.1 + other.1))
    }
}

// VE + RE
impl RosyAdd<&RE> for &VE {
    type Output = VE;
    fn rosy_add(self, other: &RE) -> Result<Self::Output> {
        Ok(self.iter().map(|x| x + other).collect())
    }
}
// VE + VE
impl RosyAdd<&VE> for &VE {
    type Output = VE;
    fn rosy_add(self, other: &VE) -> Result<Self::Output> {
        Ok(self.iter()
            .zip(other.iter())
            .map(|(x, y)| x + y)
            .collect())
    }
}

// DA + RE
impl RosyAdd<&RE> for &DA {
    type Output = DA;
    fn rosy_add(self, other: &RE) -> Result<Self::Output> {
        self + *other
    }
}

// DA + DA
impl RosyAdd<&DA> for &DA {
    type Output = DA;
    fn rosy_add(self, other: &DA) -> Result<Self::Output> {
        self + other
    }
}

// RE + CD
impl RosyAdd<&CD> for &RE {
    type Output = CD;
    fn rosy_add(self, other: &CD) -> Result<Self::Output> {
        // Create DA from real, then CD from that DA
        let self_da = DA::constant(*self);
        let self_cd = CD::from_da(&self_da);
        self_cd.rosy_add(other)
    }
}

// LO + LO (Logical OR)
impl RosyAdd<&LO> for &LO {
    type Output = LO;
    fn rosy_add(self, other: &LO) -> Result<Self::Output> {
        Ok(*self || *other)
    }
}

// CM + RE
impl RosyAdd<&DA> for &CM {
    type Output = CD;
    fn rosy_add(self, other: &DA) -> Result<Self::Output> {
        use num_complex::Complex64;
        // Create CD from the complex number
        let cm_cd = CD::complex_constant(Complex64::new(self.0, self.1));
        // Create CD from the DA (which becomes the real part)
        let da_cd = CD::from_da(other);
        // Add them
        &cm_cd + &da_cd
    }
}

// CM + CD
impl RosyAdd<&CD> for &CM {
    type Output = CD;
    fn rosy_add(self, other: &CD) -> Result<Self::Output> {
        use num_complex::Complex64;
        other + Complex64::new(self.0, self.1)
    }
}

// DA + CM
impl RosyAdd<&CM> for &DA {
    type Output = CD;
    fn rosy_add(self, other: &CM) -> Result<Self::Output> {
        use num_complex::Complex64;
        // Create CD from the complex number
        let cm_cd = CD::complex_constant(Complex64::new(other.0, other.1));
        // Create CD from the DA (which becomes the real part)
        let da_cd = CD::from_da(self);
        // Add them
        &da_cd + &cm_cd
    }
}

// DA + CD
impl RosyAdd<&CD> for &DA {
    type Output = CD;
    fn rosy_add(self, other: &CD) -> Result<Self::Output> {
        // Create CD from DA (real part only, imaginary is zero)
        let self_cd = CD::from_da(self);
        &self_cd + other
    }
}

// CD + RE
impl RosyAdd<&RE> for &CD {
    type Output = CD;
    fn rosy_add(self, other: &RE) -> Result<Self::Output> {
        use num_complex::Complex64;
        self + Complex64::new(*other, 0.0)
    }
}

// CD + CM
impl RosyAdd<&CM> for &CD {
    type Output = CD;
    fn rosy_add(self, other: &CM) -> Result<Self::Output> {
        use num_complex::Complex64;
        self + Complex64::new(other.0, other.1)
    }
}

// CD + DA
impl RosyAdd<&DA> for &CD {
    type Output = CD;
    fn rosy_add(self, other: &DA) -> Result<Self::Output> {
        // Create CD from DA (real part only, imaginary is zero)
        let other_cd = CD::from_da(other);
        self + &other_cd
    }
}

// CD + CD
impl RosyAdd<&CD> for &CD {
    type Output = CD;
    fn rosy_add(self, other: &CD) -> Result<Self::Output> {
        self + other
    }
}