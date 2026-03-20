use std::collections::HashMap;

use crate::rosy_lib::{IntrinsicTypeRule, RosyType};
use crate::rosy_lib::{RE, CM, VE, DA, CD};

/// Type registry for EXP intrinsic function.
/// 
/// According to COSY INFINITY manual, EXP supports:
/// - RE -> RE
/// - CM -> CM
/// - VE -> VE (elementwise)
/// - DA -> DA (Taylor composition)
pub const EXP_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "1.5"),
    IntrinsicTypeRule::new("CM", "CM", "CM(1.5&2.5)"),
    IntrinsicTypeRule::new("VE", "VE", "1.5&2.5&3.5"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1)"),
    IntrinsicTypeRule::new("CD", "CD", "CD(1)"),
];

/// Get the return type of EXP for a given input type.
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

/// Trait for computing the exponential of ROSY data types.
pub trait RosyEXP {
    type Output;
    fn rosy_exp(&self) -> anyhow::Result<Self::Output>;
}

/// EXP for real numbers
impl RosyEXP for RE {
    type Output = RE;
    fn rosy_exp(&self) -> anyhow::Result<Self::Output> {
        Ok(self.exp())
    }
}

/// EXP for complex numbers
impl RosyEXP for CM {
    type Output = CM;
    fn rosy_exp(&self) -> anyhow::Result<Self::Output> {
        Ok(self.exp())
    }
}

/// EXP for vectors (elementwise)
impl RosyEXP for VE {
    type Output = VE;
    fn rosy_exp(&self) -> anyhow::Result<Self::Output> {
        Ok(self.iter().map(|x| x.exp()).collect())
    }
}

/// EXP for DA (Taylor composition)
impl RosyEXP for DA {
    type Output = DA;
    fn rosy_exp(&self) -> anyhow::Result<Self::Output> {
        da_exp(self)
    }
}

/// EXP for CD (complex Taylor composition)
impl RosyEXP for CD {
    type Output = CD;
    fn rosy_exp(&self) -> anyhow::Result<Self::Output> {
        cd_exp(self)
    }
}

/// Compute exponential of a DA object using Taylor series composition.
/// 
/// Uses: exp(f) = exp(f₀) * exp(δf)
/// where exp(δf) = 1 + δf + (δf)²/2! + (δf)³/3! + ...
fn da_exp(da: &DA) -> anyhow::Result<DA> {
    use crate::rosy_lib::taylor::DACoefficient;
    
    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;
    
    let f0 = da.constant_part();
    let exp_f0 = f0.exp();
    
    // Create DA with constant part removed (δf = f - f₀)
    let mut da_prime = da.clone();
    da_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), 0.0);
    
    // Build exp(δf) = 1 + δf + (δf)²/2! + ...
    let mut result = DA::from_coeff(1.0);
    let mut term = da_prime.clone();
    let mut factorial = 1.0;
    
    for n in 1..=max_order {
        factorial *= n as f64;
        
        // Add current term: (δf)^n / n!
        let scaled_term = (&term * DA::from_coeff(1.0 / factorial))?;
        result = (&result + &scaled_term)?;
        
        // Prepare next term
        if n < max_order {
            term = (&term * &da_prime)?;
        }
    }
    
    // Multiply by exp(f₀)
    result = (&result * DA::from_coeff(exp_f0))?;
    
    Ok(result)
}

/// Compute exponential of a CD object using Taylor series composition.
fn cd_exp(cd: &CD) -> anyhow::Result<CD> {
    use crate::rosy_lib::taylor::DACoefficient;
    use num_complex::Complex64;
    
    let config = crate::rosy_lib::taylor::get_config()?;
    let max_order = config.max_order as usize;
    
    let f0 = cd.constant_part();
    let exp_f0 = f0.exp();
    
    // Create CD with constant part removed
    let mut cd_prime = cd.clone();
    cd_prime.set_coeff(crate::rosy_lib::taylor::Monomial::constant(), Complex64::zero());
    
    // Build exp(δf) = 1 + δf + (δf)²/2! + ...
    let mut result = CD::from_coeff(Complex64::one());
    let mut term = cd_prime.clone();
    let mut factorial = 1.0;
    
    for n in 1..=max_order {
        factorial *= n as f64;
        
        let scale = Complex64::new(1.0 / factorial, 0.0);
        let scaled_term = (&term * CD::from_coeff(scale))?;
        result = (&result + &scaled_term)?;
        
        if n < max_order {
            term = (&term * &cd_prime)?;
        }
    }
    
    // Multiply by exp(f₀)
    result = (&result * CD::from_coeff(exp_f0))?;
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rosy_lib::intrinsics::test_utils::test_intrinsic_output_match;

    #[test]
    fn test_rosy_cosy_exp_match() {
        test_intrinsic_output_match("exp");
    }
}
