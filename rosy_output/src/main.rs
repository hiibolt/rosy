#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use rosy_lib::*;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    // <INJECT_START>
	let mut X_ST: String = String::new();
	let mut X_RE: f64 = 0.0;
	let mut X_VE: Vec<f64> = vec!();
	let mut X_CM: (f64, f64) = (0.0, 0.0);
	X_ST = String::from("Hello World").to_owned();
	println!("{}{}", (String::from("X_ST: ")).rosy_display(), (&X_ST).rosy_display());
	X_RE = 3f64.to_owned();
	println!("{}{}", (String::from("X_RE: ")).rosy_display(), ((&X_RE).rosy_to_string().context("...while trying to convert to string!")?).rosy_display());
	X_VE = ((&X_RE)).concat((&4f64)).to_owned();
	println!("{}{}", (String::from("X_VE: ")).rosy_display(), (&X_VE).rosy_display());
	X_CM = X_VE.cm().context("...while trying to convert to a CM!")?.to_owned();
	println!("{}{}", (String::from("X_CM: ")).rosy_display(), (&X_CM).rosy_display());
	println!("{}", (String::from("")).rosy_display());
	X_ST = String::from("Another string").to_owned();
	fn PLUS_ONE ( A: &f64 ) -> Result<f64> {
		let mut PLUS_ONE: f64 = 0.0;
		PLUS_ONE = (&*A).rosy_add((&1f64)).to_owned();
		Ok(PLUS_ONE)
	}
	println!("{}{}", (String::from("PLUS_ONE(5): ")).rosy_display(), (PLUS_ONE((&5f64)).with_context(|| format!("...while trying to call function PLUS_ONE!"))?).rosy_display());
	println!("{}", (String::from("")).rosy_display());
	let mut X_LO: bool = false;
	X_LO = true.to_owned();
	if X_LO {
		println!("{}", (String::from("X_LO is TRUE")).rosy_display());
	} else {
		println!("{}", (String::from("X_LO is FALSE")).rosy_display());
	}
	println!("{}", (String::from("")).rosy_display());
	let mut STEP: f64 = 0.0;
	STEP = 2f64.to_owned();
	for I in ((0f64 as usize)..=(8f64 as usize)).step_by(STEP as usize) {
		println!("{}{}", (String::from("In loop, index I is: ")).rosy_display(), ((&I).rosy_to_string().context("...while trying to convert to string!")?).rosy_display());
	}
	// <INJECT_END>
    
    Ok(())
}