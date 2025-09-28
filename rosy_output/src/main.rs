#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use rosy_lib::*;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    // <INJECT_START>
	fn ADDTWONUMS ( X: &f64, Y: &f64 ) -> Result<f64> {
		let mut ADDTWONUMS: f64;
		println!("{}{}", String::from("X: ").rosy_display(), X.rosy_display());
		println!("{}{}", String::from("Y: ").rosy_display(), Y.rosy_display());
		let mut Z: Vec<f64> = vec!();
		ADDTWONUMS = (X.rosy_add(Y)).to_owned();
		Ok(ADDTWONUMS)
	}
	fn PRINTSEVENNUMS (  ) -> Result<()> {
		let mut X: Vec<f64> = vec!();
		let mut Y: Vec<f64> = vec!();
		Y = (&Y).concat(&8f64).to_owned();
		X = (&(&0f64.rosy_add(&1f64))).concat(&2f64).concat(&3f64).concat(&4f64).concat(&5f64).concat(&6f64).concat(&7f64).concat(&Y).to_owned();
		println!("{}", X.rosy_display());
		Ok(())
	}
	fn PRINTACOMPLEXNUM (  ) -> Result<()> {
		let mut X: Vec<f64> = vec!();
		let mut Y: (f64, f64);
		X = (&2f64).concat(&1f64).to_owned();
		Y = X.cm().context("...while trying to convert to a CM!")?.to_owned();
		println!("{}", Y.rosy_display());
		Ok(())
	}
	fn run (  ) -> Result<()> {
		let mut X: f64;
		let mut Y: f64;
		X = 3f64.to_owned();
		Y = 4f64.to_owned();
		println!("{}{}", String::from("Summation of 3 and 4: ").rosy_display(), ADDTWONUMS(&X, &Y).with_context(|| format!("...while trying to call function ADDTWONUMS!"))?.rosy_display());
		PRINTSEVENNUMS().with_context(|| format!("...while trying to call procedure PRINTSEVENNUMS!"))?;
		PRINTACOMPLEXNUM().with_context(|| format!("...while trying to call procedure PRINTACOMPLEXNUM!"))?;
		let mut I: f64;
		for I in ((0f64 as usize)..=(4f64 as usize)).step_by(2f64 as usize) {
			println!("{}", I.rosy_display());
		}
		Ok(())
	}
	// <INJECT_END>
    
    run()?;
    Ok(())
}