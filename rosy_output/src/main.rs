#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use rosy_lib::*;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    // <INJECT_START>
	fn RUN (  ) -> Result<()> {
		let mut X: Vec<Vec<Vec<Vec<f64>>>> = vec![vec![vec![vec![0.0; (4f64) as usize]; (3f64) as usize]; (2f64) as usize]; (1f64) as usize];
		for I in (1f64 as usize)..=(1f64 as usize) {
			let mut I = I as f64;
			for J in (1f64 as usize)..=(2f64 as usize) {
				let mut J = J as f64;
				for K in (1f64 as usize)..=(3f64 as usize) {
					let mut K = K as f64;
					for L in (1f64 as usize)..=(4f64 as usize) {
						let mut L = L as f64;
						X[(*((&I)) - 1.0) as usize][(*((&J)) - 1.0) as usize][(*((&K)) - 1.0) as usize][(*((&L)) - 1.0) as usize] = (&((&((&I).rosy_add((&J)))).rosy_add((&K)))).rosy_add((&L)).to_owned();
					}
				}
			}
		}
		println!("{}{}", (String::from("X: ")).rosy_display(), (X[(*((&1f64)) - 1.0) as usize][(*((&2f64)) - 1.0) as usize][(*((&3f64)) - 1.0) as usize][(*((&4f64)) - 1.0) as usize]).rosy_display());
		Ok(())
	}
	RUN().with_context(|| format!("...while trying to call procedure RUN!"))?;
	// <INJECT_END>
    
    Ok(())
}