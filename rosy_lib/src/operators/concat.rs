use std::collections::HashMap;
use crate::RosyType;

use super::super::{RE, ST, VE};

/*
Allowed operation type combinations for concatenation:

Left Right Result Comment
RE RE VE Concatenate two Reals to a Vector
RE VE VE Append a Real to the left of a Vector
ST ST ST Concatenate two Strings
VE RE VE Append a Real to the right of a Vector
VE VE VE Concatenate two Vectors
*/

pub fn get_return_type ( lhs: &RosyType, rhs: &RosyType ) -> Option<RosyType> {
    let registry: HashMap<(RosyType, RosyType), RosyType> = {
        let mut m = HashMap::new();
        let all = vec!(
            (RosyType::RE(), RosyType::RE(), RosyType::VE()),
            (RosyType::RE(), RosyType::VE(), RosyType::VE()),
            (RosyType::ST(), RosyType::ST(), RosyType::ST()),
            (RosyType::VE(), RosyType::RE(), RosyType::VE()),
            (RosyType::VE(), RosyType::VE(), RosyType::VE()),
        );
        for (left, right, result) in all {
            m.insert((left, right), result);
        }
        m
    };

    registry.get(&(*lhs, *rhs)).copied()
}

pub trait RosyConcat<T> {
    type Output;
    fn rosy_concat(self, other: T) -> Self::Output;
}
// RE + RE
impl RosyConcat<&RE> for &RE {
    type Output = VE;
    fn rosy_concat(self, other: &RE) -> Self::Output {
        vec![*self, *other]
    }
}
// RE + VE
impl RosyConcat<&VE> for &RE {
    type Output = VE;
    fn rosy_concat(self, other: &VE) -> Self::Output {
        let mut result = vec![*self];
        result.extend_from_slice(other);
        result
    }
}

// ST + ST
impl RosyConcat<&ST> for &ST {
    type Output = ST;
    fn rosy_concat(self, other: &ST) -> Self::Output {
        format!("{}{}", self, other)
    }
}

// VE + RE
impl RosyConcat<&RE> for &VE {
    type Output = VE;
    fn rosy_concat(self, other: &RE) -> Self::Output {
        let mut result = self.clone();
        result.push(*other);
        result
    }
}
// VE + VE
impl RosyConcat<&VE> for &VE {
    type Output = VE;
    fn rosy_concat(self, other: &VE) -> Self::Output {
        let mut result = self.clone();
        result.extend_from_slice(other);
        result
    }
}