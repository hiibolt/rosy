#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use rosy_lib::*;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    // <INJECT_START>
	let mut NO: f64 = 0.0;
	let mut NV: f64 = 0.0;
	let mut LX: f64 = 0.0;
	let mut X1: String = String::new();
	let mut X2: String = String::new();
	let mut X3: String = String::new();
	let mut IM: (f64, f64) = (0.0, 0.0);
	let mut COMPLEX_X1: (f64, f64) = (0.0, 0.0);
	let mut A: f64 = 0.0;
	let mut B: f64 = 0.0;
	fn PAUSE (  ) -> Result<()> {
		let mut I: f64 = 0.0;
		println!("{}", (String::from("[PAUSE - press ENTER to continue in interactive mode]")).rosy_display());
		Ok(())
	}
	fn COSYST ( X1: &mut String, X2: &mut String ) -> Result<()> {
		println!("{}", (String::from("")).rosy_display());
		println!("{}", (String::from("** ROSY Strings **")).rosy_display());
		println!("{}", (String::from("")).rosy_display());
		*X1 = String::from("Hello World!").to_owned();
		*X2 = ((&String::from("World"))).concat((&String::from(" "))).concat((&String::from("Hello"))).to_owned();
		println!("{}{}{}", (&*X2).rosy_display(), (String::from(" ")).rosy_display(), (&*X1).rosy_display());
		println!("{}", (String::from("")).rosy_display());
		println!("{}", (String::from("The above string was crafted by basic concatenation.")).rosy_display());
		PAUSE().with_context(|| format!("...while trying to call procedure PAUSE!"))?;
		Ok(())
	}
	fn COSYLO (  ) -> Result<()> {
		let mut L1: bool = false;
		let mut L2: bool = false;
		println!("{}", (String::from("")).rosy_display());
		println!("{}", (String::from("** ROSY Logicals **")).rosy_display());
		println!("{}", (String::from("")).rosy_display());
		L1 = true.to_owned();
		L2 = false.to_owned();
		if L1 {
			println!("{}", (String::from("L1 is TRUE")).rosy_display());
		}
		if L2 {
			println!("{}", (String::from("L2 is TRUE")).rosy_display());
		} else {
			println!("{}", (String::from("L2 is FALSE")).rosy_display());
		}
		PAUSE().with_context(|| format!("...while trying to call procedure PAUSE!"))?;
		Ok(())
	}
	fn COSYCM ( A: &mut f64, B: &mut f64, COMPLEX_X1: &mut (f64, f64), IM: &mut (f64, f64) ) -> Result<()> {
		println!("{}", (String::from("")).rosy_display());
		println!("{}", (String::from("** ROSY Complex numbers **")).rosy_display());
		println!("{}", (String::from("")).rosy_display());
		*IM = ((&0f64)).concat((&1f64)).cm().context("...while trying to convert to a CM!")?.to_owned();
		*COMPLEX_X1 = (&*A).concat(&*B).cm().context("...while trying to convert to a CM!")?.to_owned();
		println!("{}{}{}", (String::from("IM := CM(0&1) is ")).rosy_display(), ((&*IM).rosy_to_string().context("...while trying to convert to string!")?).rosy_display(), (String::from("  : imaginary unit")).rosy_display());
		println!("{}{}", (String::from("COMPLEX_X1 := CM(A&B) is ")).rosy_display(), ((&*COMPLEX_X1).rosy_to_string().context("...while trying to convert to string!")?).rosy_display());
		println!("{}", (String::from("")).rosy_display());
		println!("{}{}", (String::from("Extracting the real      part of COMPLEX_X1: ")).rosy_display(), (((&*COMPLEX_X1).rosy_extract((&1f64)).context("...while trying to extract component!")?).rosy_to_string().context("...while trying to convert to string!")?).rosy_display());
		println!("{}{}", (String::from("Extracting the imaginary part of COMPLEX_X1: ")).rosy_display(), (((&*COMPLEX_X1).rosy_extract((&2f64)).context("...while trying to extract component!")?).rosy_to_string().context("...while trying to convert to string!")?).rosy_display());
		PAUSE().with_context(|| format!("...while trying to call procedure PAUSE!"))?;
		Ok(())
	}
	fn COSYVE ( A: &mut f64, B: &mut f64 ) -> Result<()> {
		let mut V1: Vec<f64> = vec!();
		let mut V2: Vec<f64> = vec!();
		println!("{}", (String::from("")).rosy_display());
		println!("{}", (String::from("** ROSY Vectors **")).rosy_display());
		println!("{}", (String::from("")).rosy_display());
		V1 = (&*A).concat(&*B).to_owned();
		V2 = (&V1).concat(&V1).to_owned();
		println!("{}{}", (String::from("V1 := A & B creates: ")).rosy_display(), (&V1).rosy_display());
		println!("{}{}", (String::from("V2 := V1 & V1 creates: ")).rosy_display(), (&V2).rosy_display());
		println!("{}", (String::from("")).rosy_display());
		PAUSE().with_context(|| format!("...while trying to call procedure PAUSE!"))?;
		Ok(())
	}
	fn RUN ( A: &mut f64, B: &mut f64, COMPLEX_X1: &mut (f64, f64), IM: &mut (f64, f64), X1: &mut String, X2: &mut String ) -> Result<()> {
		*A = 2f64.to_owned();
		*B = 3f64.to_owned();
		println!("{}", (String::from("")).rosy_display());
		println!("{}{}{}{}", (String::from("A=")).rosy_display(), (&*A).rosy_display(), (String::from("  B=")).rosy_display(), (&*B).rosy_display());
		println!("{}", (String::from("")).rosy_display());
		COSYST(X1, X2).with_context(|| format!("...while trying to call procedure COSYST!"))?;
		COSYLO().with_context(|| format!("...while trying to call procedure COSYLO!"))?;
		COSYCM(A, B, COMPLEX_X1, IM).with_context(|| format!("...while trying to call procedure COSYCM!"))?;
		COSYVE(A, B).with_context(|| format!("...while trying to call procedure COSYVE!"))?;
		Ok(())
	}
	NO = 3f64.to_owned();
	NV = 2f64.to_owned();
	LX = 80f64.to_owned();
	println!("{}{}", (String::from("The estimated variable size LX is ")).rosy_display(), (&LX).rosy_display());
	RUN(&mut A, &mut B, &mut COMPLEX_X1, &mut IM, &mut X1, &mut X2).with_context(|| format!("...while trying to call procedure RUN!"))?;
	// <INJECT_END>
    
    Ok(())
}