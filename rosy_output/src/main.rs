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

    // <INJECT_START>
	let mut X: f64 = 0.0;
	let mut Y: f64 = 0.0;
	X = (&mut 2f64).to_owned();
	Y = (&mut -3f64).to_owned();
	println!("{}", &mut RosyST::rosy_to_string(&*&mut RosyAdd::rosy_add(&*&mut X, &*&mut Y)));
	// <INJECT_END>
    
    Ok(())
}