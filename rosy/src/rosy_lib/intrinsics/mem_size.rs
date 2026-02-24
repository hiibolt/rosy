use crate::rosy_lib::{RE, VE};

/// LST(n) - String memory allocation estimator (COSY compatibility).
/// In COSY, LST(n) returns n (the size in 8-byte words for a string of length n).
/// In Rosy, we just pass through the value since Rosy handles memory automatically.
pub trait RosyLST {
    fn rosy_lst(&self) -> RE;
}

impl RosyLST for RE {
    fn rosy_lst(&self) -> RE {
        *self
    }
}

/// LCM(n) - Complex number memory estimator (COSY compatibility).
/// In COSY, LCM(n) returns 2*n (complex needs 2x the real storage).
/// In Rosy, we just return 2*n for compatibility.
pub trait RosyLCM {
    fn rosy_lcm(&self) -> RE;
}

impl RosyLCM for RE {
    fn rosy_lcm(&self) -> RE {
        2.0 * self
    }
}

/// LCD(ve) - DA memory size estimator (COSY compatibility).
/// Takes a VE with (order, num_vars) and returns estimated DA memory size.
/// In COSY, LCD(NO&NV) computes (NO+NV)! / (NO! * NV!) + some overhead.
/// We compute the binomial coefficient (NO+NV choose NV) + 5 for metadata.
pub trait RosyLCD {
    fn rosy_lcd(&self) -> RE;
}

impl RosyLCD for VE {
    fn rosy_lcd(&self) -> RE {
        if self.len() < 2 {
            return 1.0;
        }
        let no = self[0] as u64;
        let nv = self[1] as u64;
        
        // Compute binomial coefficient (no + nv) choose nv
        // = (no+nv)! / (no! * nv!)
        let mut result: u64 = 1;
        for i in 1..=nv {
            result = result * (no + i) / i;
        }
        
        // Add metadata overhead (matching COSY behavior)
        (result + 5) as f64
    }
}
