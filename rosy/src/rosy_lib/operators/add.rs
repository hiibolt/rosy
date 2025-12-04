//! Addition operator for ROSY types.
//!
//! This module provides the `RosyAdd` trait and implementations for all
//! supported type combinations. The compatibility rules are defined in the
//! `ADD_REGISTRY` constant below.
//!
//! # Type Compatibility
//! 
//! See `assets/operators/add/add_table.md` for the full compatibility table.
//!
//! # Examples
//! 
//! See `assets/operators/add/add.rosy` for ROSY examples and 
//! `assets/operators/add/add.fox` for equivalent COSY INFINITY code.

use anyhow::Result;
use crate::rosy_lib::RosyType;
use crate::rosy_lib::{RE, CM, VE, DA, CD, LO};
use crate::rosy_lib::operators::{TypeRule, build_type_registry};

/// Type compatibility registry for addition operator.
/// 
/// This is the single source of truth for what type combinations are allowed.
/// The build script (`build.rs`) parses this to generate:
/// - Documentation table (`add_table.md`)
/// - ROSY test script (`add.rosy`)
/// - COSY test script (`add.fox`)
/// - Integration tests
pub const ADD_REGISTRY: &[TypeRule] = &[
    TypeRule::new("RE", "RE", "RE", "-2", "1"),
    TypeRule::new("RE", "CM", "CM", "2", "CM(0&1)"),
    TypeRule::with_comment("RE", "VE", "VE", "1", "1&2", "Add Real componentwise"),
    TypeRule::new("RE", "DA", "DA", "3", "DA(1)"),
    TypeRule::new("RE", "CD", "CD", "4", "DA(1)+CM(0&1)*DA(2)"),
    TypeRule::with_comment("LO", "LO", "LO", "LO(1)", "LO(0)", "Logical OR"),
    TypeRule::new("CM", "RE", "CM", "CM(0&1)", "5"),
    TypeRule::new("CM", "CM", "CM", "CM(2&3)", "CM(4&5)"),
    TypeRule::new("CM", "DA", "CD", "CM(0&1)", "DA(1)"),
    TypeRule::new("CM", "CD", "CD", "CM(0&1)", "DA(1)+CM(2&3)*DA(2)"),
    TypeRule::with_comment("VE", "RE", "VE", "1&2", "6", "Add Real componentwise"),
    TypeRule::with_comment("VE", "VE", "VE", "1&2", "3&4", "Add componentwise"),
    TypeRule::new("DA", "RE", "DA", "DA(1)", "7"),
    TypeRule::new("DA", "CM", "CD", "DA(1)", "CM(0&1)"),
    TypeRule::new("DA", "DA", "DA", "DA(2)", "DA(3)"),
    TypeRule::new("DA", "CD", "CD", "DA(1)", "DA(1)+CM(0&1)*DA(2)"),
    TypeRule::new("CD", "RE", "CD", "DA(1)+CM(0&1)*DA(2)", "-8"),
    TypeRule::new("CD", "CM", "CD", "DA(1)+CM(0&1)*DA(2)", "CM(2&3)"),
    TypeRule::new("CD", "DA", "CD", "DA(1)+CM(0&1)*DA(2)", "DA(3)"),
    TypeRule::new("CD", "CD", "CD", "DA(1)+CM(0&1)*DA(2)", "DA(3)+CM(4&5)*DA(6)"),
];

pub fn get_return_type(lhs: &RosyType, rhs: &RosyType) -> Option<RosyType> {
    let registry = build_type_registry(ADD_REGISTRY);
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
        let self_da = DA::from_coeff(*self);
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
        Ok((&cm_cd + &da_cd)?)
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

#[cfg(test)]
mod tests {
    use super::super::test_utils::test_operator_output_match;

    #[test]
    fn test_rosy_cosy_output_match() {
        test_operator_output_match("add");
    }
}