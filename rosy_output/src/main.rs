#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use rosy_lib::*;

use anyhow::{Result, Context};

fn main() -> Result<()> {
    // <INJECT_START>
	fn ADDTWONUMS ( X: &f64, Y: &f64 ) -> f64 {
		let mut ADDTWONUMS: f64 = 0.0;
		println!("{}{}", &mut "X: ".to_string(), &mut RosyST::rosy_to_string(&*X));
		println!("{}{}", &mut "Y: ".to_string(), &mut RosyST::rosy_to_string(&*Y));
		let mut Z: Vec<f64> = vec![];
		ADDTWONUMS = (&mut RosyAdd::rosy_add(&*X, &*Y)).to_owned();
		ADDTWONUMS
	}
	fn PRINTSEVENNUMS (  ) {
		let mut X: Vec<f64> = vec![];
		let mut Y: Vec<f64> = vec![];
		Y = (&mut RosyConcat::rosy_concat(&*&mut Y, &*&mut 8f64)).to_owned();
		X = (&mut RosyConcat::rosy_concat(&*&mut RosyConcat::rosy_concat(&*&mut RosyConcat::rosy_concat(&*&mut RosyConcat::rosy_concat(&*&mut RosyConcat::rosy_concat(&*&mut RosyConcat::rosy_concat(&*&mut RosyConcat::rosy_concat(&*&mut RosyAdd::rosy_add(&*&mut 0f64, &*&mut 1f64), &*&mut 2f64), &*&mut 3f64), &*&mut 4f64), &*&mut 5f64), &*&mut 6f64), &*&mut 7f64), &*&mut Y)).to_owned();
		println!("{}", &mut RosyST::rosy_to_string(&*&mut X));
	}
	fn PRINTACOMPLEXNUM (  ) {
		let mut X: Vec<f64> = vec![];
		let mut Y: (f64, f64) = (0.0, 0.0);
		X = (&mut RosyConcat::rosy_concat(&*&mut 2f64, &*&mut 1f64)).to_owned();
		println!("{}", &mut RosyST::rosy_to_string(&*&mut Y));
	}
	fn RUN (  ) {
		let mut X: f64 = 0.0;
		let mut Y: f64 = 0.0;
		X = (&mut 3f64).to_owned();
		Y = (&mut 4f64).to_owned();
		println!("{}{}", &mut "Summation of 3 and 4: ".to_string(), &mut RosyST::rosy_to_string(&*&ADDTWONUMS(&mut X, &mut Y)));
		PRINTSEVENNUMS();
		PRINTACOMPLEXNUM();
		for I in (((&mut 0f64).to_owned() as usize)..=((&mut 4f64).to_owned() as usize)).step_by((&mut 2f64).to_owned() as usize) {
			let mut I = I as RE;
			println!("{}", &mut RosyST::rosy_to_string(&*&mut I));
		}
	}
	RUN();
	// <INJECT_END>
    
    Ok(())
}