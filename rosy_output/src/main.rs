#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use rosy_lib::*;

use anyhow::{Result, Context};

fn main() -> Result<()> {
    // <INJECT_START>
	let mut counter: f64 = 0.0;
	let mut global_message: String = "".to_string();
	let mut operation_complete: bool = false;
	fn INCREMENT_COUNTER ( counter: &mut f64 ) {
		*counter = (&mut RosyAdd::rosy_add(&*counter, &*&mut 1f64)).to_owned();
		println!("{}{}", &mut "Counter incremented to: ".to_string(), &mut RosyST::rosy_to_string(&*counter));
	}
	fn SET_MESSAGE ( global_message: &mut String, message: &mut String ) {
		*global_message = (message).to_owned();
		println!("{}{}", &mut "Global message set to: ".to_string(), global_message);
	}
	fn MARK_COMPLETE ( operation_complete: &mut bool ) {
		*operation_complete = (&mut true).to_owned();
		println!("{}", &mut "Operation marked as complete".to_string());
	}
	fn DISPLAY_GLOBALS ( counter: &mut f64, global_message: &mut String, operation_complete: &mut bool ) {
		println!("{}", &mut "=== Current Global State ===".to_string());
		println!("{}{}", &mut "Counter: ".to_string(), &mut RosyST::rosy_to_string(&*counter));
		println!("{}{}", &mut "Message: ".to_string(), global_message);
		if (operation_complete).to_owned() {
			println!("{}", &mut "Status: COMPLETE".to_string());
		} else {
			println!("{}", &mut "Status: INCOMPLETE".to_string());
		}
		println!("{}", &mut "========================".to_string());
	}
	fn RUN ( counter: &mut f64, global_message: &mut String, operation_complete: &mut bool ) {
		*counter = (&mut 0f64).to_owned();
		*global_message = (&mut "Initial message".to_string()).to_owned();
		*operation_complete = (&mut false).to_owned();
		println!("{}", &mut "Initial state:".to_string());
		DISPLAY_GLOBALS(counter, global_message, operation_complete);
		INCREMENT_COUNTER(counter);
		INCREMENT_COUNTER(counter);
		INCREMENT_COUNTER(counter);
		SET_MESSAGE(global_message, &mut &mut "Hello from global variables!".to_string());
		MARK_COMPLETE(operation_complete);
		println!("{}", &mut "Final state:".to_string());
		DISPLAY_GLOBALS(counter, global_message, operation_complete);
		if (operation_complete).to_owned() {
			println!("{}", &mut "Operation is complete!".to_string());
		}
	}
	RUN(&mut counter, &mut global_message, &mut operation_complete);
	// <INJECT_END>
    
    Ok(())
}