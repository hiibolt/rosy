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
    IntrinsicTypeRule::new("CM", "CM", "1.5&2.5"),
    IntrinsicTypeRule::new("VE", "VE", "1.5&2.5&3.5"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1)"),
    IntrinsicTypeRule::new("CD", "CD", "CM(1.5&2.5)+CD(1)"),
];

/// Get the return type of SIN for a given input type.
pub fn get_return_type(input: &RosyType) -> Option<RosyType> {
    let registry: HashMap<RosyType, RosyType> = {
        let mut m = HashMap::new();
        let all = vec![
            (RosyType::RE(), RosyType::RE()),
            (RosyType::CM(), RosyType::CM()),
            (RosyType::VE(), RosyType::VE()),
            (RosyType::DA(), RosyType::DA()),
            (RosyType::CD(), RosyType::CD()),
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
/// Uses the formula: sin(f(x)) = sin(f₀) + cos(f₀)·f'(x) - sin(f₀)·(f'(x))²/2! - ...
/// where f₀ is the constant part of the DA.
fn da_sin(da: &DA) -> anyhow::Result<DA> {
    use crate::rosy_lib::taylor::DACoefficient;
    
    let f0 = da.constant_part();
    let sin_f0 = f0.sin();
    let cos_f0 = f0.cos();
    
    // Create DA with constant part removed (this is f'(x) in Taylor sense)
    let mut da_prime = da.clone();
    da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);
    
    // Build result using Taylor composition
    // sin(f) = sin(f0) + cos(f0)*(f-f0) - sin(f0)*(f-f0)^2/2! - cos(f0)*(f-f0)^3/3! + ...
    let mut result = DA::from_coeff(sin_f0);
    let mut term = da_prime.clone();
    let mut coefficient = cos_f0;
    let mut factorial = 1.0;
    let mut sign_sin = true; // alternates: cos, -sin, -cos, sin, cos, ...
    
    for n in 1..=5 { // Match COSY's order-5 expansion
        factorial *= n as f64;
        
        // Add current term
        let scaled_term = (&term * DA::from_coeff(coefficient / factorial))?;
        result = (&result + &scaled_term)?;
        
        // Prepare next term
        term = (&term * &da_prime)?;
        
        // Update coefficient (alternating sin/cos with sign changes)
        coefficient = if sign_sin {
            -sin_f0
        } else {
            -cos_f0
        };
        sign_sin = !sign_sin;
    }
    
    Ok(result)
}

/// Compute sine of a CD object using Taylor series composition.
fn cd_sin(cd: &CD) -> anyhow::Result<CD> {
    use crate::rosy_lib::taylor::DACoefficient;
    use num_complex::Complex64;
    
    let f0 = cd.constant_part();
    let sin_f0 = f0.sin();
    let cos_f0 = f0.cos();
    
    // Create CD with constant part removed
    let mut cd_prime = cd.clone();
    cd_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), Complex64::zero());
    
    // Build result using Taylor composition
    let mut result = CD::from_coeff(sin_f0);
    let mut term = cd_prime.clone();
    let mut coefficient = cos_f0;
    let mut factorial = 1.0;
    let mut sign_sin = true;
    
    for n in 1..=5 {
        factorial *= n as f64;
        
        let scaled_term = (&term * CD::from_coeff(coefficient / factorial))?;
        result = (&result + &scaled_term)?;
        
        term = (&term * &cd_prime)?;
        
        coefficient = if sign_sin {
            -sin_f0
        } else {
            -cos_f0
        };
        sign_sin = !sign_sin;
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rosy_lib::intrinsics::test_utils::test_intrinsic_output_match;

    #[test]
    #[ignore] // Run after codegen creates test files
    fn test_rosy_cosy_sin_match() {
        test_intrinsic_output_match("sin");
    }
}
