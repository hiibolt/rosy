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
	let mut MEOW_2D: Vec<Vec<f64>> = vec![vec![0.0; (&2f64).to_owned() as usize]; (&1f64).to_owned() as usize];
	fn RUN ( MEOW2: &mut f64, MEOW_VAR: &mut Vec<f64>, MEOW1: &mut f64, NOT_MEOW_2D: &mut Vec<Vec<f64>> ) {
		let mut MEOW2: f64 = 0.0;
		let mut MEOW_VAR: Vec<f64> = vec![0.0; (&MEOW2.rosy_add(&1f64)).to_owned() as usize];
		fn NESTED ( MEOW1: &mut f64, MEOW2: &mut f64, MEOW_VAR: &mut Vec<f64> ) {
			let mut X: Vec<Vec<Vec<Vec<f64>>>> = vec![vec![vec![vec![0.0; (&4f64).to_owned() as usize]; (&3f64).to_owned() as usize]; ((&*MEOW2).rosy_add(&1f64)).to_owned() as usize]; ((&*MEOW1)).to_owned() as usize];
			X[(&1f64).to_owned() as usize][(&2f64).to_owned() as usize][(&3f64).to_owned() as usize] = ((&*MEOW_VAR)).to_owned();
		}
		MEOW_VAR[(&0f64).to_owned() as usize] = (&5f64).to_owned();
	}
	fn ADD_TWO ( B: &mut f64, A: &mut f64 ) -> f64 {
		let mut ADD_TWO: f64 = 0.0;
		ADD_TWO = ((&*A).rosy_add((&*B))).to_owned();
		ADD_TWO
	}
	// <INJECT_END>
    
    Ok(())
}