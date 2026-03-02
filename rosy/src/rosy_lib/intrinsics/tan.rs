use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, VE, DA, CD};

/// Type registry for TAN intrinsic function.
/// 
/// According to COSY INFINITY manual, TAN supports:
/// - RE -> RE
/// - VE -> VE (elementwise)
/// - DA -> DA (Taylor composition)
/// 
/// Note: CM is NOT supported for TAN in COSY.
pub const TAN_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "1.5"),
    IntrinsicTypeRule::new("VE", "VE", "1.5&2.5&3.5"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1)"),
];

/// Get the return type of TAN for a given input type.
pub fn get_return_type(input: &RosyType) -> Option<RosyType> {
    let registry: HashMap<RosyType, RosyType> = {
        let mut m = HashMap::new();
        let all = vec![
            (RosyType::RE(), RosyType::RE()),
            (RosyType::VE(), RosyType::VE()),
            (RosyType::DA(), RosyType::DA()),
        ];
        for (input_type, result_type) in all {
            m.insert(input_type, result_type);
        }
        m
    };

    registry.get(input).copied()
}

/// Trait for computing the tangent of ROSY data types.
pub trait RosyTAN {
    type Output;
    fn rosy_tan(&self) -> anyhow::Result<Self::Output>;
}

/// TAN for real numbers
impl RosyTAN for RE {
    type Output = RE;
    fn rosy_tan(&self) -> anyhow::Result<Self::Output> {
        Ok(self.tan())
    }
}

/// TAN for vectors (elementwise)
impl RosyTAN for VE {
    type Output = VE;
    fn rosy_tan(&self) -> anyhow::Result<Self::Output> {
        Ok(self.iter().map(|x| x.tan()).collect())
    }
}

/// TAN for DA (Taylor composition)
/// Uses: tan(f) = sin(f) / cos(f)
impl RosyTAN for DA {
    type Output = DA;
    fn rosy_tan(&self) -> anyhow::Result<Self::Output> {
        use crate::rosy_lib::taylor::DACoefficient;
        use crate::rosy_lib::intrinsics::sin::RosySIN;
        
        let config = crate::rosy_lib::taylor::get_config()?;
        let max_order = config.max_order as usize;
        
        let f0 = self.constant_part();
        let tan_f0 = f0.tan();
        let sec2_f0 = 1.0 / (f0.cos() * f0.cos());  // sec²(f₀)
        
        // Create DA with constant part removed (δf = f - f₀)
        let mut da_prime = self.clone();
        da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);
        
        // For tan, we use the Taylor series approach
        // tan(f₀ + δf) where we compute term by term
        // Using the identity: d/dx tan(x) = sec²(x) = 1 + tan²(x)
        // We build up coefficients using the recurrence relation
        
        // Alternative: just compute sin(f)/cos(f) using existing implementations
        let sin_f = self.rosy_sin()?;
        let cos_f = da_cos(self)?;
        
        // Divide sin by cos
        (&sin_f / &cos_f).map_err(|e| e)
    }
}

/// Compute cosine of a DA for internal use by TAN.
fn da_cos(da: &DA) -> anyhow::Result<DA> {
    use crate::rosy_lib::taylor::DACoefficient;
    
    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;
    
    let f0 = da.constant_part();
    let cos_f0 = f0.cos();
    let sin_f0 = f0.sin();
    
    // Create DA with constant part removed
    let mut da_prime = da.clone();
    da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);
    
    // Build cos(f₀ + δf) using Taylor series
    // cos(f₀ + δf) = cos(f₀) - sin(f₀)·δf - cos(f₀)·(δf)²/2! + sin(f₀)·(δf)³/3! + ...
    let mut result = DA::from_coeff(cos_f0);
    let mut term = da_prime.clone();
    let mut factorial = 1.0;
    
    // Cycle: -sin, -cos, sin, cos, -sin, -cos, sin, cos, ...
    let coeffs = [-sin_f0, -cos_f0, sin_f0, cos_f0];
    
    for n in 1..=max_order {
        factorial *= n as f64;
        let coeff_index = (n - 1) % 4;
        let coefficient = coeffs[coeff_index];
        
        let scaled_term = (&term * DA::from_coeff(coefficient / factorial))?;
        result = (&result + &scaled_term)?;
        
        if n < max_order {
            term = (&term * &da_prime)?;
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rosy_lib::intrinsics::test_utils::test_intrinsic_output_match;

    #[test]
    fn test_rosy_cosy_tan_match() {
        test_intrinsic_output_match("tan");
    }
}
