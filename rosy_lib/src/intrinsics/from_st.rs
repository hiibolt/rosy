
use std::collections::HashMap;

use anyhow::{Result, Context};

use crate::RosyType;


pub fn get_return_type ( input: &RosyType ) -> Option<RosyType> {
    let registry: HashMap<RosyType, RosyType> = {
        let mut m = HashMap::new();
        let all = vec!(
            (RosyType::ST(), RosyType::RE()),
            (RosyType::ST(), RosyType::LO()),
            (RosyType::ST(), RosyType::ST()),
        );
        for (input, result) in all {
            m.insert(input, result);
        }
        m
    };

    registry.get(input).copied()
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