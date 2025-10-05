#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use rosy_lib::*;
use anyhow::{Result, Context, ensure, bail};

fn main() -> Result<()> {
	let rosy_mpi_context = RosyMPIContext::new()
		.context("Failed to initialize Rosy MPI context")?;

    // <INJECT_START>
	let mut NP: f64 = 0.0;
	NP = (&mut 100f64).to_owned();
	let mut X: Vec<f64> = vec![0.0; (&mut NP).to_owned() as usize];
	{
		let mut I = rosy_mpi_context.get_group_num(&mut &mut NP)? + 1.0f64;
	
		X[((&mut I).to_owned() - 1.0f64) as usize] = (&mut RosyAdd::rosy_add(&*&mut I, &*&mut 11f64)).to_owned();
	
		rosy_mpi_context.coordinate(&mut X, 1u8, &mut &mut NP)?;
	}
	println!("{}", &mut "Final values of X:".to_string());
	for I in (((&mut 1f64).to_owned() as usize)..=((&mut NP).to_owned() as usize)) {
		let mut I = I as RE;
		println!("{}{}{}{}", &mut "X[".to_string(), &mut RosyST::rosy_to_string(&*&mut I), &mut "] = ".to_string(), &mut RosyST::rosy_to_string(&*&mut X[((&mut I).to_owned() - 1.0f64) as usize]));
	}
	// <INJECT_END>
    
    Ok(())
}