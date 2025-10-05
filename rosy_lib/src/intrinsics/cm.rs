use std::collections::HashMap;

use crate::RosyType;
use super::super::{RE, CM, VE};
use anyhow::{Result, ensure};

pub fn get_return_type ( lhs: &RosyType ) -> Option<RosyType> {
    let registry: HashMap<RosyType, RosyType> = {
        let mut m = HashMap::new();
        let all = vec!(
            (RosyType::RE(), RosyType::CM()),
            (RosyType::CM(), RosyType::CM()),
            (RosyType::VE(), RosyType::CM()),
        );
        for (left, result) in all {
            m.insert(left, result);
        }
        m
    };

    registry.get(&*lhs).copied()
}


pub trait RosyCM {
    fn rosy_cm(self) -> Result<CM>;
}
// RE -> CM
impl RosyCM for &RE {
    fn rosy_cm(self) -> Result<CM> {
        Ok((*self, 0.0))
    }
}
// CM -> CM
impl RosyCM for &CM {
    fn rosy_cm(self) -> Result<CM> {
        Ok(*self)
    }
}
// VE -> CM
impl RosyCM for &VE {
    fn rosy_cm(self) -> Result<CM> {
        ensure!(self.len() == 2, "Cannot convert vector of length {} to CM (complex), must have exactly 2 elements!", self.len());

        Ok((self[0], self[1]))
    }
}