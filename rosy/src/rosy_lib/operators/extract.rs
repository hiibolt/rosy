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
use crate::rosy_lib::{RE, ST, VE, CM, DA, CD};
use crate::rosy_lib::operators::{TypeRule, build_type_registry};
use crate::rosy_lib::taylor::monomial::Monomial;

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
/// as documented in manual.md Section A.2.
pub const EXTRACT_REGISTRY: &[TypeRule] = &[
    TypeRule::with_comment("ST", "RE", "ST", "'test'", "2", "Extract i-th character"),
    TypeRule::with_comment("ST", "VE", "ST", "'test'", "2&3", "Extract substring by range"),
    TypeRule::with_comment("CM", "RE", "RE", "CM(3&4)", "1", "Extract real part"),
    TypeRule::with_comment("VE", "RE", "RE", "1&2", "2", "Extract i-th component"),
    TypeRule::with_comment("VE", "VE", "VE", "1&2&3", "2&3", "Extract subvector by range"),
    TypeRule::with_comment("DA", "RE", "RE", "DA(1)", "1", "Extract DA coefficient by flat index"),
    TypeRule::with_comment("DA", "VE", "RE", "DA(1)", "0&1", "Extract DA coefficient by exponent vector"),
    TypeRule::with_comment("CD", "RE", "CM", "CD(1)", "1", "Extract CD coefficient by flat index"),
    TypeRule::with_comment("CD", "VE", "CM", "CD(1)", "0&1", "Extract CD coefficient by exponent vector"),
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
            1 => Ok(self.re), // Real part
            2 => Ok(self.im), // Imaginary part
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

// DA | RE -> RE (extract DA coefficient by flat monomial index — not commonly used)
impl RosyExtract<&RE> for &DA {
    type Output = RE;

    fn rosy_extract(self, _index: &RE) -> Result<Self::Output> {
        // Flat index extraction for DA is rarely used in practice.
        // COSY's flat index maps to monomials in graded lexicographic order.
        // For now, index 0 returns the constant part.
        let idx = *_index as usize;
        if idx == 0 {
            return Ok(self.constant_part());
        }
        // For other flat indices, enumerate monomials in graded lex order
        let config = crate::rosy_lib::taylor::get_config()
            .map_err(|e| anyhow::anyhow!("DA extraction requires initialized Taylor: {}", e))?;
        let all_monomials = crate::rosy_lib::taylor::monomial::enumerate_monomials(
            config.max_order, config.num_vars as u32
        );
        if idx >= all_monomials.len() {
            bail!("DA flat index {} out of bounds (0-{})", idx, all_monomials.len() - 1);
        }
        Ok(self.get_coeff(&all_monomials[idx]))
    }
}

// DA | VE -> RE (extract DA coefficient by exponent vector)
impl RosyExtract<&VE> for &DA {
    type Output = RE;

    fn rosy_extract(self, index: &VE) -> Result<Self::Output> {
        let config = crate::rosy_lib::taylor::get_config()
            .map_err(|e| anyhow::anyhow!("DA extraction requires initialized Taylor: {}", e))?;
        if index.len() > config.num_vars as usize {
            bail!(
                "Exponent vector length {} exceeds number of DA variables {}",
                index.len(), config.num_vars
            );
        }
        let mut exponents = [0u8; crate::rosy_lib::taylor::MAX_VARS];
        for (i, &val) in index.iter().enumerate() {
            exponents[i] = val as u8;
        }
        let monomial = Monomial::new(exponents);
        Ok(self.get_coeff(&monomial))
    }
}

// CD | RE -> CM (extract CD coefficient by flat monomial index)
impl RosyExtract<&RE> for &CD {
    type Output = CM;

    fn rosy_extract(self, _index: &RE) -> Result<Self::Output> {
        let idx = *_index as usize;
        if idx == 0 {
            return Ok(self.constant_part());
        }
        let config = crate::rosy_lib::taylor::get_config()
            .map_err(|e| anyhow::anyhow!("CD extraction requires initialized Taylor: {}", e))?;
        let all_monomials = crate::rosy_lib::taylor::monomial::enumerate_monomials(
            config.max_order, config.num_vars as u32
        );
        if idx >= all_monomials.len() {
            bail!("CD flat index {} out of bounds (0-{})", idx, all_monomials.len() - 1);
        }
        Ok(self.get_coeff(&all_monomials[idx]))
    }
}

// CD | VE -> CM (extract CD coefficient by exponent vector)
impl RosyExtract<&VE> for &CD {
    type Output = CM;

    fn rosy_extract(self, index: &VE) -> Result<Self::Output> {
        let config = crate::rosy_lib::taylor::get_config()
            .map_err(|e| anyhow::anyhow!("CD extraction requires initialized Taylor: {}", e))?;
        if index.len() > config.num_vars as usize {
            bail!(
                "Exponent vector length {} exceeds number of CD variables {}",
                index.len(), config.num_vars
            );
        }
        let mut exponents = [0u8; crate::rosy_lib::taylor::MAX_VARS];
        for (i, &val) in index.iter().enumerate() {
            exponents[i] = val as u8;
        }
        let monomial = Monomial::new(exponents);
        Ok(self.get_coeff(&monomial))
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::test_operator_output_match;

    #[test]
    fn test_rosy_cosy_output_match() {
        test_operator_output_match("extract");
    }
}