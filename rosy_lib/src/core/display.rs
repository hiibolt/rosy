use super::super::{RE, ST, LO, CM, VE};

pub trait RosyDisplay {
    fn rosy_display(self) -> String;
}
impl RosyDisplay for &RE {
    fn rosy_display(self) -> String {
        self.to_string()
    }
}

impl RosyDisplay for &ST {
    fn rosy_display(self) -> String {
        self.to_string()
    }
}

impl RosyDisplay for &LO {
    fn rosy_display(self) -> String {
        let s = if *self { "TRUE" } else { "FALSE" };

        s.to_string()
    }
}

impl RosyDisplay for &CM {
    fn rosy_display(self) -> String {
        format!("({} + {}i)", self.0, self.1)
    }
}

impl RosyDisplay for &VE {
    fn rosy_display(self) -> String {
        let elements: Vec<String> = self.iter()
            .map(|x| x.to_string())
            .collect();
        format!("[{}]", elements.join(", "))
    }
}

// Required as loops cast to `usize`
impl RosyDisplay for &usize {
    fn rosy_display(self) -> String {
        self.to_string()
    }
}

impl RosyDisplay for &str {
    fn rosy_display(self) -> String {
        self.to_string()
    }
}