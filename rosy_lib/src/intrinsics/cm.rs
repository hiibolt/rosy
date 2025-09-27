use anyhow::{Result, ensure};

use super::super::{RE, CE, VE};

pub trait RosyCM {
    fn cm(self) -> Result<CE>;
}
// RE -> CE
impl RosyCM for &RE {
    fn cm(self) -> Result<CE> {
        Ok((*self, 0.0))
    }
}
// CE -> CE
impl RosyCM for &CE {
    fn cm(self) -> Result<CE> {
        Ok(*self)
    }
}
// VE -> CE
impl RosyCM for &VE {
    fn cm(self) -> Result<CE> {
        ensure!(self.len() == 2, "Cannot convert vector of length {} to CM (complex), must have exactly 2 elements!", self.len());

        Ok((self[0], self[1]))
    }
}