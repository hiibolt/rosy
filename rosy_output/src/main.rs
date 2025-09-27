#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use rosy_lib::*;
use anyhow::{Result, Context};

// <INJECT_START>
	fn ADDTWONUMS ( X: &f64, Y: &f64 ) -> Result<f64> {
		let mut ADDTWONUMS: f64;
		println!("{:?}{:?}", String::from("X: "), X);
		println!("{:?}{:?}", String::from("Y: "), Y);
		ADDTWONUMS = (X.rosy_add(Y)).to_owned();
		Ok(ADDTWONUMS)
	}
	fn PRINTSEVENNUMS (  ) -> Result<()> {
		let mut X: Vec<f64>;
		X = (&(&0f64.rosy_add(&1f64))).concat(&2f64).concat(&3f64).concat(&4f64).concat(&5f64).concat(&6f64).concat(&7f64).to_owned();
		println!("{:?}", X);
		Ok(())
	}
	fn PRINTACOMPLEXNUM (  ) -> Result<()> {
		let mut X: Vec<f64>;
		let mut Y: (f64, f64);
		X = (&2f64).concat(&1f64).to_owned();
		Y = X.cm().context("...while trying to convert to a CM!")?.to_owned();
		println!("{:?}", Y);
		Ok(())
	}
	fn PRINTYOURNUMBER (  ) -> Result<()> {
		let mut I: f64;
		I = from_stdin().context("...while trying to read from stdin!")?;
		println!("{:?}{:?}", String::from("I = "), I);
		Ok(())
	}
	fn run (  ) -> Result<()> {
		let mut X: f64;
		let mut Y: f64;
		X = 3f64.to_owned();
		Y = 4f64.to_owned();
		println!("{:?}{:?}", String::from("Summation of 3 and 4: "), ADDTWONUMS(&X, &Y));
		PRINTSEVENNUMS().with_context(|| format!("...while trying to call procedure PRINTSEVENNUMS!"))?;
		PRINTACOMPLEXNUM().with_context(|| format!("...while trying to call procedure PRINTACOMPLEXNUM!"))?;
		PRINTYOURNUMBER().with_context(|| format!("...while trying to call procedure PRINTYOURNUMBER!"))?;
		let mut I: f64;
		for I in ((0f64 as usize)..=(4f64 as usize)).step_by(2f64 as usize) {
			println!("{:?}", I);
		}
		Ok(())
	}
	// <INJECT_END>

fn main () -> Result<()> {
    run()
        .context("Encountered a runtime error")?;

    Ok(())
}