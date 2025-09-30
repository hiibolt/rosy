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
		let mut X: Vec<Vec<Vec<Vec<f64>>>> = vec![vec![vec![vec![0.0; (1f64) as usize]; (2f64) as usize]; (3f64) as usize]; (4f64) as usize];
		println!("{}{}", (String::from("X: ")).rosy_display(), (&X).rosy_display());
		Ok(())
	}
	RUN().with_context(|| format!("...while trying to call procedure RUN!"))?;
	// <INJECT_END>
    
    Ok(())
}