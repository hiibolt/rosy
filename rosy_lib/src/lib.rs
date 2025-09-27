mod operators;
mod intrinsics;

pub use operators::*;
pub use intrinsics::*;

pub type RE = f64;
pub type ST = String;
pub type LO = bool;
pub type CM = (f64, f64);
pub type VE = Vec<f64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RosyType {
    RE,
    ST,
    LO,
    CM,
    VE,
}
impl RosyType {
    pub fn as_rust_type(&self) -> &'static str {
        match self {
            RosyType::RE => "f64",
            RosyType::ST => "String",
            RosyType::LO => "bool",
            RosyType::CM => "(f64, f64)",
            RosyType::VE => "Vec<f64>",
        }
    }
}
impl TryFrom<&str> for RosyType {
    type Error = anyhow::Error;
    fn try_from( value: &str ) -> Result<RosyType, Self::Error> {
        match value {
            "(RE)" => Ok(RosyType::RE),
            "(ST)" => Ok(RosyType::ST),
            "(LO)" => Ok(RosyType::LO),
            "(CM)" => Ok(RosyType::CM),
            "(VE)" => Ok(RosyType::VE),
            _ => Err(anyhow::anyhow!("Can't convert {} to a ROSY type", value)),
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 */