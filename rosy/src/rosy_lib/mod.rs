pub mod operators;
pub mod intrinsics;
pub mod core;
pub mod mpi;
pub mod taylor;
pub mod macros;

pub use operators::*;
pub use intrinsics::*;
pub use core::*;
pub use mpi::*;

// Re-export dace types (legacy - will be replaced by taylor module)
pub use dace::DA as DaceDA;

// New taylor types
pub use taylor::{DA, CD};

pub type RE = f64;
pub type ST = String;
pub type LO = bool;
pub type CM = (f64, f64);
pub type VE = Vec<f64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RosyType {
    pub base_type: RosyBaseType,
    pub dimensions: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RosyBaseType {
    RE,
    ST,
    LO,
    CM,
    VE,
    DA,
    CD,
}
impl std::fmt::Display for RosyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.dimensions == 0 {
            write!(f, "({:?})", self.base_type)
        } else {
            let dims = "*".repeat(self.dimensions);
            write!(f, "({:?} {dims})", self.base_type)
        }
    }
}
impl RosyType {
    pub fn new ( base_type: RosyBaseType, dimensions: usize ) -> Self {
        RosyType {
            base_type,
            dimensions
        }
    }

    #[allow(non_snake_case)]
    pub fn RE ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::RE,
            dimensions: 0
        }
    }
    #[allow(non_snake_case)]
    pub fn ST ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::ST,
            dimensions: 0
        }
    }
    #[allow(non_snake_case)]
    pub fn LO ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::LO,
            dimensions: 0
        }
    }
    #[allow(non_snake_case)]
    pub fn CM ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::CM,
            dimensions: 0
        }
    }
    #[allow(non_snake_case)]
    pub fn VE ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::VE,
            dimensions: 0
        }
    }
    #[allow(non_snake_case)]
    pub fn DA ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::DA,
            dimensions: 0
        }
    }
    #[allow(non_snake_case)]
    pub fn CD ( ) -> Self {
        RosyType {
            base_type: RosyBaseType::CD,
            dimensions: 0
        }
    }

    pub fn as_rust_type (&self) -> String {
        let base = match self.base_type {
            RosyBaseType::RE => "f64",
            RosyBaseType::ST => "String",
            RosyBaseType::LO => "bool",
            RosyBaseType::CM => "(f64, f64)",
            RosyBaseType::VE => "Vec<f64>",
            RosyBaseType::DA => "DA",
            RosyBaseType::CD => "CD",
        }.to_string();

        if self.dimensions == 0 {
            base
        } else {
            let mut result = base;
            for _ in 0..self.dimensions {
                result = format!("Vec<{}>", result);
            }
            result
        }
    }
}
impl TryFrom<&str> for RosyBaseType {
    type Error = anyhow::Error;
    fn try_from( value: &str ) -> Result<RosyBaseType, Self::Error> {
        match value {
            "RE" => Ok(RosyBaseType::RE),
            "ST" => Ok(RosyBaseType::ST),
            "LO" => Ok(RosyBaseType::LO),
            "CM" => Ok(RosyBaseType::CM),
            "VE" => Ok(RosyBaseType::VE),
            "DA" => Ok(RosyBaseType::DA),
            "CD" => Ok(RosyBaseType::CD),
            _ => Err(anyhow::anyhow!("Can't convert {} to a ROSY type", value)),
        }
    }
}