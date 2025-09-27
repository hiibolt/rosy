use super::super::{RE, CE, VE};

pub trait RosyAdd<Rhs = Self> {
    type Output;
    fn rosy_add(self, rhs: Rhs) -> Self::Output;
}
// RE + RE
impl RosyAdd<&RE> for &RE {
    type Output = RE;
    fn rosy_add(self, rhs: &RE) -> Self::Output {
        self + rhs
    }
}
// RE + CE
impl RosyAdd<&CE> for &RE {
    type Output = CE;
    fn rosy_add(self, other: &CE) -> Self::Output {
        (self + other.0, other.1)
    }
}
// RE + VE
impl RosyAdd<&VE> for &RE {
    type Output = VE;
    fn rosy_add(self, other: &VE) -> Self::Output {
        other.iter().map(|x| x + self).collect()
    }
}

// CE + RE
impl RosyAdd<&RE> for &CE {
    type Output = CE;
    fn rosy_add(self, other: &RE) -> Self::Output {
        (self.0 + other, self.1)
    }
}
// CE + CE
impl RosyAdd<&CE> for &CE {
    type Output = CE;
    fn rosy_add(self, other: &CE) -> Self::Output {
        (self.0 + other.0, self.1 + other.1)
    }
}

// VE + RE
impl RosyAdd<&RE> for &VE {
    type Output = VE;
    fn rosy_add(self, other: &RE) -> Self::Output {
        self.iter().map(|x| x + other).collect()
    }
}
// VE + VE
impl RosyAdd<&VE> for &VE {
    type Output = VE;
    fn rosy_add(self, other: &VE) -> Self::Output {
        self.iter()
            .zip(other.iter())
            .map(|(x, y)| x + y)
            .collect()
    }
}