#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use rosy_lib::*;

use anyhow::{Result, Context};

fn main() -> Result<()> {
    // <INJECT_START>
	let mut test_string: String = "".to_string();
	let mut test_vector: Vec<f64> = vec![];
	let mut test_complex: (f64, f64) = (0.0, 0.0);
	test_string = (&mut "Hello".to_string()).to_owned();
	println!("{}{}", &mut "String: ".to_string(), &mut test_string);
	println!("{}{}", &mut "Character 1: ".to_string(), &mut RosyST::rosy_to_string(&*&mut RosyExtract::rosy_extract(&*&mut test_string, &*&mut 1f64).context("...while trying to extract an element")?));
	println!("{}{}", &mut "Character 3: ".to_string(), &mut RosyST::rosy_to_string(&*&mut RosyExtract::rosy_extract(&*&mut test_string, &*&mut 3f64).context("...while trying to extract an element")?));
	test_vector = (&mut RosyConcat::rosy_concat(&*&mut RosyConcat::rosy_concat(&*&mut 10f64, &*&mut 20f64), &*&mut 30f64)).to_owned();
	println!("{}{}", &mut "Vector: ".to_string(), &mut RosyST::rosy_to_string(&*&mut test_vector));
	println!("{}{}", &mut "Element 1: ".to_string(), &mut RosyST::rosy_to_string(&*&mut RosyExtract::rosy_extract(&*&mut test_vector, &*&mut 1f64).context("...while trying to extract an element")?));
	println!("{}{}", &mut "Element 2: ".to_string(), &mut RosyST::rosy_to_string(&*&mut RosyExtract::rosy_extract(&*&mut test_vector, &*&mut 2f64).context("...while trying to extract an element")?));
	// <INJECT_END>
    
    Ok(())
}