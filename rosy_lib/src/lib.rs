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

use anyhow::{Result, Context};
pub fn from_stdin<T: std::str::FromStr> ( ) -> Result<T>
where
    <T as std::str::FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    use std::io;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Failed to read line from stdin!")?;

    let input = input.trim();
    let value = input
        .parse::<T>()
        .map_err(|e| anyhow::anyhow!("Failed to parse input '{}': {}", input, e))?;

    Ok(value)
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
