//! Extraction operator for ROSY types.
//!
//! This module provides the `RosyExtract` trait and implementations for all
//! supported type combinations. The compatibility rules are defined in the
//! `EXTRACT_REGISTRY` constant below.
//!
//! # Type Compatibility
//! 
//! See `assets/operators/extract/extract_table.md` for the full compatibility table.
//!
//! # Examples
//! 
//! See `assets/operators/extract/extract.rosy` for ROSY examples and 
//! `assets/operators/extract/extract.fox` for equivalent COSY INFINITY code.

use anyhow::{Result, bail};

use crate::rosy_lib::RosyType;
use crate::rosy_lib::{RE, ST, VE, CM};
use crate::rosy_lib::operators::{TypeRule, build_type_registry};

/// Type compatibility registry for extraction operator.
/// 
/// This is the single source of truth for what type combinations are allowed.
/// The build script (`build.rs`) parses this to generate:
/// - Documentation table (`extract_table.md`)
/// - ROSY test script (`extract.rosy`)
/// - COSY test script (`extract.fox`)
/// - Integration tests
/// 
/// This registry matches COSY INFINITY's | operator capabilities exactly,
/// as documented in manual.md Section A.2. Note that we currently skip:
/// - RE | RE => RE (no effect)
/// - RE | VE => RE (no effect)
/// - DA | RE => RE (DA type not fully integrated)
/// - DA | VE => RE (DA type not fully integrated)
/// - CD | RE => CM (CD type not fully integrated)
/// - CD | VE => CM (CD type not fully integrated)
pub const EXTRACT_REGISTRY: &[TypeRule] = &[
    TypeRule::with_comment("ST", "RE", "ST", "Extract the i-th character"),
    TypeRule::with_comment("ST", "VE", "ST", "Extract character range (two-vector)"),
    TypeRule::with_comment("CM", "RE", "RE", "Extract component (1: real, 2: imaginary)"),
    TypeRule::with_comment("VE", "RE", "RE", "Extract the i-th component"),
    TypeRule::with_comment("VE", "VE", "VE", "Extract component range (two-vector)"),
];

pub fn get_return_type(base: &RosyType, index: &RosyType) -> Option<RosyType> {
    let registry = build_type_registry(EXTRACT_REGISTRY);
    registry.get(&(*base, *index)).copied()
}

/// Trait for extracting components from ROSY data types
pub trait RosyExtract<T> {
    type Output;
    fn rosy_extract(self, index: T) -> Result<Self::Output>;
}

// ST | RE -> ST (extract i-th character)
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

// ST | VE -> ST (extract substring by range)
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

// CM | RE -> RE (extract real or imaginary part)
impl RosyExtract<&RE> for &CM {
    type Output = RE;
    
    fn rosy_extract(self, index: &RE) -> Result<Self::Output> {
        match *index as i32 {
            1 => Ok(self.0), // Real part
            2 => Ok(self.1), // Imaginary part
            _ => bail!("Complex number index must be 1 (real) or 2 (imaginary), found {}", index),
        }
    }
}

// VE | RE -> RE (extract i-th component)
impl RosyExtract<&RE> for &VE {
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

// VE | VE -> VE (extract subvector by range)
impl RosyExtract<&VE> for &VE {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_st_re() -> Result<()> {
        let s = "hello".to_string();
        let result = (&s).rosy_extract(&1.0)?;
        assert_eq!(result, "h");
        Ok(())
    }

    #[test]
    fn test_st_ve() -> Result<()> {
        let s = "hello".to_string();
        let result = (&s).rosy_extract(&vec![2.0, 4.0])?;
        assert_eq!(result, "ell");
        Ok(())
    }

    #[test]
    fn test_cm_re() -> Result<()> {
        let c = (3.0, 4.0);
        let real = (&c).rosy_extract(&1.0)?;
        let imag = (&c).rosy_extract(&2.0)?;
        assert_eq!(real, 3.0);
        assert_eq!(imag, 4.0);
        Ok(())
    }

    #[test]
    fn test_ve_re() -> Result<()> {
        let v = vec![1.0, 2.0, 3.0];
        let result = (&v).rosy_extract(&2.0)?;
        assert_eq!(result, 2.0);
        Ok(())
    }

    #[test]
    fn test_ve_ve() -> Result<()> {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = (&v).rosy_extract(&vec![2.0, 4.0])?;
        assert_eq!(result, vec![2.0, 3.0, 4.0]);
        Ok(())
    }
}