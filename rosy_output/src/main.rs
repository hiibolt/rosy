#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use rosy_lib::*;

use anyhow::{Result, Context};

fn main() -> Result<()> {
    // <INJECT_START>
	fn RUN (  ) {
		let mut condition1: bool = false;
		let mut condition2: bool = false;
		let mut number: f64 = 0.0;
		condition1 = (&true).to_owned();
		condition2 = (&false).to_owned();
		number = (&42f64).to_owned();
		if (&condition1).to_owned() {
			println!("{}", &"First condition is true".to_string());
		} else if (&condition2).to_owned() {
			println!("{}", &"Second condition is true".to_string());
		} else {
			println!("{}", &"Neither condition is true".to_string());
		}
		if (&condition2).to_owned() {
			println!("{}", &"This should not print".to_string());
		} else if (&condition1).to_owned() {
			println!("{}", &"This should print - ELSEIF works!".to_string());
		} else {
			println!("{}", &"This else should not print".to_string());
		}
		if (&condition2).to_owned() {
			println!("{}", &"False condition".to_string());
		} else {
			println!("{}", &"ELSE clause works!".to_string());
		}
		println!("{}{}", &"Number is: ".to_string(), (&number).rosy_to_string());
	}
	RUN();
	// <INJECT_END>
    
    Ok(())
}