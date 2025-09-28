use anyhow::Result;

/// Trait for converting ROSY data types to strings
pub trait RosyToString {
    fn rosy_to_string(&self) -> Result<String>;
}

/// Convert owned f64 to string
impl RosyToString for f64 {
    fn rosy_to_string(&self) -> Result<String> {
        Ok(self.to_string())
    }
}

/// Convert real numbers to strings
impl RosyToString for &f64 {
    fn rosy_to_string(&self) -> Result<String> {
        Ok(self.to_string())
    }
}

/// Convert strings to strings (identity)
impl RosyToString for &String {
    fn rosy_to_string(&self) -> Result<String> {
        Ok((*self).clone())
    }
}

/// Convert booleans to strings
impl RosyToString for &bool {
    fn rosy_to_string(&self) -> Result<String> {
        Ok(if **self { "TRUE".to_string() } else { "FALSE".to_string() })
    }
}

/// Convert vectors to strings
impl RosyToString for &Vec<f64> {
    fn rosy_to_string(&self) -> Result<String> {
        let elements: Vec<String> = self.iter().map(|x| x.to_string()).collect();
        Ok(format!("[{}]", elements.join(", ")))
    }
}

/// Convert complex numbers to strings
impl RosyToString for &(f64, f64) {
    fn rosy_to_string(&self) -> Result<String> {
        let (real, imag) = **self;
        if imag >= 0.0 {
            Ok(format!("({} + {}i)", real, imag))
        } else {
            Ok(format!("({} - {}i)", real, -imag))
        }
    }
}