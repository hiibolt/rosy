use std::collections::HashMap;

use anyhow::{Result, bail};

use crate::rosy_lib::{RosyType, RE, ST, VE};

/*
Left Right Result Comment
(skip) RE RE RE (no effect when the 1st component is requested)
(skip) RE VE RE (no effect when the 1st component is requested)
ST RE ST Extract the i-th component
ST VE ST Extract component range in two-vector
CM RE RE Input 1: real part, 2: imaginary part
VE RE RE Extract the i-th component
VE VE VE Extract component range in two-vector
(skip) DA RE RE Extract coefficient of 1D DA for supplied exponent
(skip) DA VE RE Extract coefficient for exponents in vector
(skip) CD RE CM Extract coefficient of 1D CD for supplied exponent
(skip) CD VE CM Extract coefficient for exponents in vector
*/


pub fn get_return_type ( base: &RosyType, index: &RosyType ) -> Option<RosyType> {
    let registry: HashMap<(RosyType, RosyType), RosyType> = {
        let mut m = HashMap::new();
        let all = vec!(
            (RosyType::ST(), RosyType::RE(), RosyType::ST()),
            (RosyType::ST(), RosyType::VE(), RosyType::ST()),
            (RosyType::CM(), RosyType::RE(), RosyType::RE()),
            (RosyType::VE(), RosyType::RE(), RosyType::RE()),
            (RosyType::VE(), RosyType::VE(), RosyType::VE()),
        );
        for (base, index, result) in all {
            m.insert((base, index), result);
        }
        m
    };

    registry.get(&(*base, *index)).copied()
}

/// Trait for extracting components from ROSY data types
pub trait RosyExtract<T> {
    type Output;
    fn rosy_extract(self, index: T) -> Result<Self::Output>;
}

/// String extraction - extract character or substring by index
impl RosyExtract<&RE> for &ST {
    type Output = ST;
    
    fn rosy_extract(self, index: &RE) -> Result<Self::Output> {
        let idx = *index as usize;
        if idx == 0 || idx > self.len() {
            bail!("String index {} out of bounds (1-{})", idx, self.len());
        }
        
        // ROSY uses 1-based indexing
        let char_at_idx = self.chars().nth(idx - 1)
            .ok_or_else(|| anyhow::anyhow!("Character at index {} not found", idx))?;
        
        Ok(char_at_idx.to_string())
    }
}

/// String extraction - extract substring by start and end indices
impl RosyExtract<&VE> for &ST {
    type Output = ST;
    
    fn rosy_extract(self, index: &VE) -> Result<Self::Output> {
        if index.len() != 2 {
            bail!("String extraction with vector index requires exactly two elements (start and end)");
        }
        
        let start = index[0] as usize;
        let end = index[1] as usize;
        
        if start == 0 || end == 0 || start > self.len() || end > self.len() || start > end {
            bail!("String index range {}-{} out of bounds (1-{})", start, end, self.len());
        }
        
        // ROSY uses 1-based indexing
        let substring: String = self.chars().skip(start - 1).take(end - start + 1).collect();
        
        Ok(substring)
    }
}

/// Vector extraction - extract component by index
impl RosyExtract<&RE> for &Vec<RE> {
    type Output = RE;
    
    fn rosy_extract(self, index: &RE) -> Result<Self::Output> {
        let idx = *index as usize;
        if idx == 0 || idx > self.len() {
            bail!("Vector index {} out of bounds (1-{})", idx, self.len());
        }
        
        // ROSY uses 1-based indexing
        Ok(self[idx - 1])
    }
}

/// Vector extraction - extract subvector by start and end indices
impl RosyExtract<&VE> for &Vec<RE> {
    type Output = VE;

    fn rosy_extract(self, index: &VE) -> Result<Self::Output> {
        if index.len() != 2 {
            bail!("Vector extraction with vector index requires exactly two elements (start and end)");
        }
        
        let start = index[0] as usize;
        let end = index[1] as usize;
        
        if start == 0 || end == 0 || start > self.len() || end > self.len() || start > end {
            bail!("Vector index range {}-{} out of bounds (1-{})", start, end, self.len());
        }
        
        // ROSY uses 1-based indexing
        Ok(self[start - 1..end].to_vec())
    }
}

/// Complex number extraction - extract real (1) or imaginary (2) component
impl RosyExtract<&f64> for &(f64, f64) {
    type Output = f64;
    
    fn rosy_extract(self, index: &f64) -> Result<Self::Output> {
        match *index as i32 {
            1 => Ok(self.0), // Real part
            2 => Ok(self.1), // Imaginary part
            _ => bail!("Complex number index must be 1 (real) or 2 (imaginary), found {}", index),
        }
    }
}