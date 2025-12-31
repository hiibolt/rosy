use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, CM, VE, DA, CD};

/// Type registry for SIN intrinsic function.
/// 
/// According to COSY INFINITY manual, SIN supports:
/// - RE -> RE
/// - CM -> CM (complex sin)
/// - VE -> VE (elementwise)
/// - DA -> DA (Taylor composition)
/// - CD -> CD (complex Taylor composition)
pub const SIN_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "1.5"),
    IntrinsicTypeRule::new("CM", "CM", "CM(1.5&2.5)"),
    IntrinsicTypeRule::new("VE", "VE", "1.5&2.5&3.5"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1)"),
];

/// Get the return type of SIN for a given input type.
pub fn get_return_type(input: &RosyType) -> Option<RosyType> {
    let registry: HashMap<RosyType, RosyType> = {
        let mut m = HashMap::new();
        let all = vec![
            (RosyType::RE(), RosyType::RE()),
            (RosyType::CM(), RosyType::CM()),
            (RosyType::VE(), RosyType::VE()),
            (RosyType::DA(), RosyType::DA())
        ];
        for (input_type, result_type) in all {
            m.insert(input_type, result_type);
        }
        m
    };

    registry.get(input).copied()
}

/// Trait for computing sine of ROSY data types.
pub trait RosySIN {
    type Output;
    fn rosy_sin(&self) -> anyhow::Result<Self::Output>;
}

/// SIN for real numbers
impl RosySIN for RE {
    type Output = RE;
    fn rosy_sin(&self) -> anyhow::Result<Self::Output> {
        Ok(self.sin())
    }
}

/// SIN for complex numbers
impl RosySIN for CM {
    type Output = CM;
    fn rosy_sin(&self) -> anyhow::Result<Self::Output> {
        Ok(self.sin())
    }
}

/// SIN for vectors (elementwise)
impl RosySIN for VE {
    type Output = VE;
    fn rosy_sin(&self) -> anyhow::Result<Self::Output> {
        Ok(self.iter().map(|x| x.sin()).collect())
    }
}

/// SIN for DA (Taylor composition)
impl RosySIN for DA {
    type Output = DA;
    fn rosy_sin(&self) -> anyhow::Result<Self::Output> {
        da_sin(self)
    }
}

/// SIN for CD (complex Taylor composition)
impl RosySIN for CD {
    type Output = CD;
    fn rosy_sin(&self) -> anyhow::Result<Self::Output> {
        cd_sin(self)
    }
}

/// Compute sine of a DA object using Taylor series composition.
/// 
/// Uses the Taylor series: sin(f) = sin(f₀) + cos(f₀)·δf - sin(f₀)·(δf)²/2! - cos(f₀)·(δf)³/3! + sin(f₀)·(δf)⁴/4! + ...
/// where f₀ is the constant part and δf = f - f₀
fn da_sin(da: &DA) -> anyhow::Result<DA> {
    use crate::rosy_lib::taylor::DACoefficient;
    
    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;
    
    let f0 = da.constant_part();
    let sin_f0 = f0.sin();
    let cos_f0 = f0.cos();
    
    // Create DA with constant part removed (δf = f - f₀)
    let mut da_prime = da.clone();
    da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);
    
    // Build result using Taylor composition
    // Pattern: sin(f₀), cos(f₀), -sin(f₀), -cos(f₀), sin(f₀), cos(f₀), ...
    let mut result = DA::from_coeff(sin_f0);
    let mut term = da_prime.clone();
    let mut factorial = 1.0;
    
    // Cycle through: cos, -sin, -cos, sin, cos, -sin, -cos, sin, ...
    let coeffs = [cos_f0, -sin_f0, -cos_f0, sin_f0];
    
    for n in 1..=max_order {
        factorial *= n as f64;
        let coeff_index = (n - 1) % 4;
        let coefficient = coeffs[coeff_index];
        
        // Add current term
        let scaled_term = (&term * DA::from_coeff(coefficient / factorial))?;
        result = (&result + &scaled_term)?;
        
        // Prepare next term
        if n < max_order {
            term = (&term * &da_prime)?;
        }
    }
    
    Ok(result)
}

/// Compute sine of a CD object using Taylor series composition.
fn cd_sin(cd: &CD) -> anyhow::Result<CD> {
    use crate::rosy_lib::taylor::DACoefficient;
    use num_complex::Complex64;
    
    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;
    
    let f0 = cd.constant_part();
    let sin_f0 = f0.sin();
    let cos_f0 = f0.cos();
    
    // Create CD with constant part removed
    let mut cd_prime = cd.clone();
    cd_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), Complex64::zero());
    
    // Build result using Taylor composition
    let mut result = CD::from_coeff(sin_f0);
    let mut term = cd_prime.clone();
    let mut factorial = 1.0;
    
    // Cycle through: cos, -sin, -cos, sin, cos, -sin, -cos, sin, ...
    let coeffs = [cos_f0, -sin_f0, -cos_f0, sin_f0];
    
    for n in 1..=max_order {
        factorial *= n as f64;
        let coeff_index = (n - 1) % 4;
        let coefficient = coeffs[coeff_index];
        
        let scaled_term = (&term * CD::from_coeff(coefficient / factorial))?;
        result = (&result + &scaled_term)?;
        
        if n < max_order {
            term = (&term * &cd_prime)?;
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rosy_lib::intrinsics::test_utils::test_intrinsic_output_match;

    #[test]
    fn test_rosy_cosy_sin_match() {
        test_intrinsic_output_match("sin");
    }
}
