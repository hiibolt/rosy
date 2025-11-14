
use std::collections::HashSet;

use anyhow::{Result, Context};

use crate::rosy_lib::RosyType;


pub fn can_be_obtained ( input: &RosyType ) -> bool {
    let registry: HashSet<_> = HashSet::from([
        RosyType::RE(),
        RosyType::ST(),
        RosyType::LO()
    ]);
    registry.contains(input)
}
pub fn from_stdin<T: RosyFromST> ( ) -> Result<T> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line from stdin!")?;
    let input = input.trim_end().to_string();
    T::rosy_from_st(input)
}

pub trait RosyFromST {
    fn rosy_from_st( st: String ) -> Result<Self>
    where
        Self: Sized;
}

impl RosyFromST for f64 {
    fn rosy_from_st( st: String ) -> Result<Self> {
        st.parse::<f64>()
            .context("Failed to parse `ST` to `RE`!")
    }
}

impl RosyFromST for String {
    fn rosy_from_st( st: String ) -> Result<Self> {
        Ok(st)
    }
}

impl RosyFromST for bool {
    fn rosy_from_st( st: String ) -> Result<Self> {
        match st.as_str() {
            "TRUE" => Ok(true),
            "FALSE" => Ok(false),
            _ => Err(anyhow::anyhow!("Failed to parse `ST` to `LO`!"))
        }
    }
}