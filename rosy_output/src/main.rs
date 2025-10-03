#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use rosy_lib::*;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    // <INJECT_START>
	let mut MEOW1: f64 = 0.0;
	MEOW1 = (&7f64).to_owned();
	let mut MEOW_2D: Vec<Vec<f64>> = vec![vec![0.0; (&2f64).to_owned() as usize]; (&1f64).to_owned() as usize];
	println!("{}{}", &"meow ".to_string(), (&MEOW1).rosy_to_string());
	fn RUN ( MEOW1: &mut f64 ) {
		let mut MEOW2: f64 = 0.0;
		let mut MEOW_VAR: Vec<f64> = vec![0.0; (&MEOW2.rosy_add(&1f64)).to_owned() as usize];
		fn NESTED ( MEOW1: &mut f64, MEOW2: &mut f64, MEOW_VAR: &mut Vec<f64> ) {
			let mut X: Vec<Vec<Vec<Vec<f64>>>> = vec![vec![vec![vec![0.0; (&4f64).to_owned() as usize]; (&3f64).to_owned() as usize]; ((&*MEOW2).rosy_add(&1f64)).to_owned() as usize]; ((&*MEOW1)).to_owned() as usize];
			X[(&1f64).to_owned() as usize][(&2f64).to_owned() as usize][(&3f64).to_owned() as usize] = ((&*MEOW_VAR)).to_owned();
			println!("{}", &"meow".to_string());
		}
		MEOW_VAR[(&0f64).to_owned() as usize] = (&5f64).to_owned();
	}
	fn ADD_TWO ( A: &f64, B: &f64 ) -> f64 {
		let mut ADD_TWO: f64 = 0.0;
		ADD_TWO = ((&*A).rosy_add((&*B))).to_owned();
		ADD_TWO
	}
	fn ADD_TWO_AND_MEOW ( MEOW1: &mut f64, A: &f64, B: &f64 ) -> f64 {
		let mut ADD_TWO_AND_MEOW: f64 = 0.0;
		ADD_TWO_AND_MEOW = ((&*A).rosy_add((&*B)).rosy_add((&*MEOW1))).to_owned();
		ADD_TWO_AND_MEOW
	}
	println!("{}{}", &"add two: ".to_string(), (&ADD_TWO_AND_MEOW(&mut MEOW1, &2f64, &2f64).rosy_add(&ADD_TWO_AND_MEOW(&mut MEOW1, &3f64, &3f64.rosy_add(&4f64)))).rosy_to_string());
	RUN(&mut MEOW1);
	// <INJECT_END>
    
    Ok(())
}