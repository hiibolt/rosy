use anyhow::{Result, ensure};

use super::super::{RE, CM, VE};

pub trait RosyCM {
    fn cm(self) -> Result<CM>;
}
// RE -> CM
impl RosyCM for &RE {
    fn cm(self) -> Result<CM> {
        Ok((*self, 0.0))
    }
}
// CM -> CM
impl RosyCM for &CM {
    fn cm(self) -> Result<CM> {
        Ok(*self)
    }
}
// VE -> CM
impl RosyCM for &VE {
    fn cm(self) -> Result<CM> {
        ensure!(self.len() == 2, "Cannot convert vector of length {} to CM (complex), must have exactly 2 elements!", self.len());

        Ok((self[0], self[1]))
    }
}