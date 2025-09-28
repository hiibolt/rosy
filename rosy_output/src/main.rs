#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use rosy_lib::*;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    // <INJECT_START>
	fn run (  ) -> Result<()> {
		let mut condition1: bool;
		let mut condition2: bool;
		let mut number: f64;
		condition1 = true.to_owned();
		condition2 = false.to_owned();
		number = 42f64.to_owned();
		if condition1 {
			println!("{}", String::from("First condition is true").rosy_display());
		} else if condition2 {
			println!("{}", String::from("Second condition is true").rosy_display());
		} else {
			println!("{}", String::from("Neither condition is true").rosy_display());
		}
		if condition2 {
			println!("{}", String::from("This should not print").rosy_display());
		} else if condition1 {
			println!("{}", String::from("This should print - ELSEIF works!").rosy_display());
		} else {
			println!("{}", String::from("This else should not print").rosy_display());
		}
		if condition2 {
			println!("{}", String::from("False condition").rosy_display());
		} else {
			println!("{}", String::from("ELSE clause works!").rosy_display());
		}
		println!("{}{}", String::from("Number is: ").rosy_display(), number.rosy_display());
		Ok(())
	}
	// <INJECT_END>
    
    run()?;
    Ok(())
}