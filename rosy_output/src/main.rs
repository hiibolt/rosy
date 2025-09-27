#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use rosy_lib::*;
use anyhow::{Result, Context};

fn run () -> Result<()> {
    // <INJECT_START>
	fn ADDTWONUMS ( X: &f64, Y: &f64 ) -> Cosy {
		let mut ADDTWONUMS: f64;
		println!("{}{}", String::from("X: "), X);
		println!("{}{}", String::from("Y: "), Y);
		ADDTWONUMS = (X.cosy_add(Y)).to_owned();
		ADDTWONUMS
	}
	fn PRINTSEVENNUMS (  ) {
		let mut X: Vec<f64>;
		X = (&(&0f64.cosy_add(&1f64))).concat(&2f64).concat(&3f64).concat(&4f64).concat(&5f64).concat(&6f64).concat(&7f64).to_owned();
		println!("{}", X);
	}
	fn PRINTACOMPLEXNUM (  ) {
		let mut X: Vec<f64>;
		let mut Y: (f64, f64);
		X = (&2f64).concat(&1f64).to_owned();
		Y = X.cm().to_owned();
		println!("{}", Y);
	}
	fn PRINTYOURNUMBER (  ) {
		let mut I: f64;
		I = Cosy::from_stdin();
		println!("{}{}", String::from("I = "), I);
	}
	fn main (  ) {
		let mut X: f64;
		let mut Y: f64;
		X = 3f64.to_owned();
		Y = 4f64.to_owned();
		println!("{}{}", String::from("Summation of 3 and 4: "), ADDTWONUMS(&X, &Y));
		PRINTSEVENNUMS();
		PRINTACOMPLEXNUM();
		PRINTYOURNUMBER();
		let mut I: f64;
		for I in (0f64.into_usize()..=4f64.into_usize()).step_by(2f64 as usize) {
		println!("{}", I);
	}
	}
	// <INJECT_END>

    Ok(())
}
fn main () -> Result<()> {
    run()
        .context("Encountered a runtime error")?;

    Ok(())
}