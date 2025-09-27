use super::super::{RE, ST, VE};


pub trait RosyConcat<T> {
    type Output;
    fn concat(self, other: T) -> Self::Output;
}
// RE + RE
impl RosyConcat<&RE> for &RE {
    type Output = VE;
    fn concat(self, other: &RE) -> Self::Output {
        vec![*self, *other]
    }
}
// RE + VE
impl RosyConcat<&VE> for &RE {
    type Output = VE;
    fn concat(self, other: &VE) -> Self::Output {
        let mut result = vec![*self];
        result.extend_from_slice(other);
        result
    }
}

// ST + ST
impl RosyConcat<&ST> for &ST {
    type Output = ST;
    fn concat(self, other: &ST) -> Self::Output {
        format!("{}{}", self, other)
    }
}

// VE + RE
impl RosyConcat<&RE> for &VE {
    type Output = VE;
    fn concat(self, other: &RE) -> Self::Output {
        let mut result = self.clone();
        result.push(*other);
        result
    }
}
// VE + VE
impl RosyConcat<&VE> for &VE {
    type Output = VE;
    fn concat(self, other: &VE) -> Self::Output {
        let mut result = self.clone();
        result.extend_from_slice(other);
        result
    }
}