//! Allowed operation type combinations for addition:
//!
//! | Left | Right | Result | Comment |
//! |---|---|---|---|
//! | RE | RE | RE | |
//! | RE | CM | CM | |
//! | RE | VE | VE | Add Real componentwise |
//! | RE | DA | DA | |
//! | RE | CD | CD | |
//! | LO | LO | LO | Logical OR |
//! | CM | RE | CM | |
//! | CM | CM | CM | |
//! | CM | DA | CD | | 
//! | CM | CD | CD | |
//! | VE | RE | VE | Add Real componentwise |
//! | VE | VE | VE | Add componentwise |
//! | DA | RE | DA | |
//! | DA | CM | CD | | 
//! | DA | DA | DA | |
//! | DA | CD | CD | | 
//! | CD | RE | CD | |
//! | CD | CM | CD | |
//! | CD | DA | CD | |
//! | CD | CD | CD | |
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
use crate::rosy_lib::{RE, CM, VE, DA, CD};

pub fn get_return_type ( lhs: &RosyType, rhs: &RosyType ) -> Option<RosyType> {
    let registry: HashMap<(RosyType, RosyType), RosyType> = {
        let mut m = HashMap::new();
        let all = vec!(
            (RosyType::RE(), RosyType::RE(), RosyType::RE()),
            (RosyType::RE(), RosyType::CM(), RosyType::CM()),
            (RosyType::RE(), RosyType::VE(), RosyType::VE()),
            (RosyType::RE(), RosyType::DA(), RosyType::DA()),
            (RosyType::CM(), RosyType::RE(), RosyType::CM()),
            (RosyType::CM(), RosyType::CM(), RosyType::CM()),
            (RosyType::VE(), RosyType::RE(), RosyType::VE()),
            (RosyType::VE(), RosyType::VE(), RosyType::VE()),
            (RosyType::DA(), RosyType::RE(), RosyType::DA()),
            (RosyType::DA(), RosyType::DA(), RosyType::DA()),
        );
        for (left, right, result) in all {
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