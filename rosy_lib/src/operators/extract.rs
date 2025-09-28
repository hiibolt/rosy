use anyhow::{Result, bail};

/// Trait for extracting components from ROSY data types
pub trait RosyExtract<T> {
    type Output;
    fn rosy_extract(&self, index: T) -> Result<Self::Output>;
}

/// String extraction - extract character or substring by index
impl RosyExtract<&f64> for &String {
    type Output = String;
    
    fn rosy_extract(&self, index: &f64) -> Result<Self::Output> {
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

/// Vector extraction - extract component by index
impl RosyExtract<&f64> for &Vec<f64> {
    type Output = f64;
    
    fn rosy_extract(&self, index: &f64) -> Result<Self::Output> {
        let idx = *index as usize;
        if idx == 0 || idx > self.len() {
            bail!("Vector index {} out of bounds (1-{})", idx, self.len());
        }
        
        // ROSY uses 1-based indexing
        Ok(self[idx - 1])
    }
}

/// Complex number extraction - extract real (1) or imaginary (2) component
impl RosyExtract<&f64> for &(f64, f64) {
    type Output = f64;
    
    fn rosy_extract(&self, index: &f64) -> Result<Self::Output> {
        match *index as i32 {
            1 => Ok(self.0), // Real part
            2 => Ok(self.1), // Imaginary part
            _ => bail!("Complex number index must be 1 (real) or 2 (imaginary), found {}", index),
        }
    }
}