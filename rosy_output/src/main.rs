#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use dace::*;
use rosy_lib::*;
use anyhow::{Result, Context, ensure, bail};

fn main() -> Result<()> {
	let start = std::time::Instant::now();
	let rosy_mpi_context = RosyMPIContext::new()
		.context("Failed to initialize Rosy MPI context")?;
	let group_num = rosy_mpi_context.get_group_num(&mut  &mut 1.0f64)
		.context("Failed to get group number")? + 1.0f64;

	let after_init = std::time::Instant::now();
    // <INJECT_START>
	let mut NP: f64 = 0.0;
	NP = (&mut 10f64).to_owned();
	let mut X: Vec<f64> = vec![0.0; (&mut NP).to_owned() as usize];
	{
		let mut I = rosy_mpi_context.get_group_num(&mut &mut NP)? + 1.0f64;
	
		for J in (((&mut 1f64).to_owned() as usize)..=((&mut 100000000f64).to_owned() as usize)) {
			let mut J = J as RE;
			X[((&mut I).to_owned() - 1.0f64) as usize] = (&mut RosyAdd::rosy_add(&*&mut RosyAdd::rosy_add(&*&mut RosyAdd::rosy_add(&*&mut X[((&mut I).to_owned() - 1.0f64) as usize], &*&mut I), &*&mut 11f64), &*&mut J)).to_owned();
		}
	
		rosy_mpi_context.coordinate(&mut X, 1u8, &mut &mut NP)?;
	}
	println!("{}", &mut "Final values of X:".to_string());
	for I in (((&mut 1f64).to_owned() as usize)..=((&mut NP).to_owned() as usize)) {
		let mut I = I as RE;
		println!("{}", &mut RosyConcat::rosy_concat(&*&mut RosyConcat::rosy_concat(&*&mut RosyConcat::rosy_concat(&*&mut "X[".to_string(), &*&mut RosyST::rosy_to_string(&*&mut I)), &*&mut "] = ".to_string()), &*&mut RosyST::rosy_to_string(&*&mut X[((&mut I).to_owned() - 1.0f64) as usize])));
	}
	// <INJECT_END>
	println!(
		"\nFinished in {}s ({}s init, {}s execution) ]",
		start.elapsed().as_secs_f64(),
		(after_init - start).as_secs_f64(),
		(start.elapsed() - (after_init - start)).as_secs_f64()
	);
    
    Ok(())
}