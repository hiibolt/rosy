use std::collections::HashMap;

use crate::RosyType;
use super::super::{RE, CM, VE, LO, ST};

pub fn get_return_type ( lhs: &RosyType ) -> Option<RosyType> {
    let registry: HashMap<RosyType, RosyType> = {
        let mut m = HashMap::new();
        let all = vec!(
            (RosyType::RE(), RosyType::ST()),
            (RosyType::ST(), RosyType::ST()),
            (RosyType::LO(), RosyType::ST()),
            (RosyType::CM(), RosyType::ST()),
            (RosyType::VE(), RosyType::ST()),
        );
        for (left, result) in all {
            m.insert(left, result);
        }
        m
    };

    registry.get(&*lhs).copied()
}           

/// Trait for converting ROSY data types to strings
pub trait RosyST {
    fn rosy_to_string(self) -> String;
}

/// Convert real numbers to strings
impl RosyST for &RE {
    fn rosy_to_string(self) -> String {
        format!(
            " {}{:.15}", 
            if self.is_sign_negative() { "-" } else { " " },
            self.abs()
        )
    }
}

/// Convert strings to strings (identity)
impl RosyST for &ST {
    fn rosy_to_string(self) -> String {
        (*self).clone()
    }
}

/// Convert booleans to strings
impl RosyST for &LO {
    fn rosy_to_string(self) -> String {
        if *self { "TRUE".to_string() } else { "FALSE".to_string() }
    }
}

/// Convert vectors to strings
impl RosyST for &VE {
    fn rosy_to_string(self) -> String {
        let elements: Vec<String> = self.iter().map(|x| x.to_string()).collect();
        format!("[{}]", elements.join(", "))
    }
}

/// Convert complex numbers to strings
impl RosyST for &CM {
    fn rosy_to_string(self) -> String {
        let (real, imag) = *self;
        if imag >= 0.0 {
            format!("({} + {}i)", real, imag)
        } else {
            format!("({} - {}i)", real, -imag)
        }
    }
}