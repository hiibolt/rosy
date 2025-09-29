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
	let mut global_message: String = String::new();
	let mut operation_complete: bool = false;
	fn INCREMENT_COUNTER ( counter: &mut f64 ) -> Result<()> {
		*counter = (&*counter).rosy_add(&1f64);
		println!("{}{}", (String::from("Counter incremented to: ")).rosy_display(), (&*counter).rosy_display());
		Ok(())
	}
	fn SET_MESSAGE ( message: &String, global_message: &mut String ) -> Result<()> {
		*global_message = (*message).clone();
		println!("{}{}", (String::from("Global message set to: ")).rosy_display(), (&*global_message).rosy_display());
		Ok(())
	}
	fn MARK_COMPLETE ( operation_complete: &mut bool ) -> Result<()> {
		*operation_complete = true;
		println!("{}", (String::from("Operation marked as complete")).rosy_display());
		Ok(())
	}
	fn DISPLAY_GLOBALS ( counter: &mut f64, global_message: &mut String, operation_complete: &mut bool ) -> Result<()> {
		println!("{}", (String::from("=== Current Global State ===")).rosy_display());
		println!("{}{}", (String::from("Counter: ")).rosy_display(), (&*counter).rosy_display());
		println!("{}{}", (String::from("Message: ")).rosy_display(), (&*global_message).rosy_display());
		if *operation_complete {
			println!("{}", (String::from("Status: COMPLETE")).rosy_display());
		} else {
			println!("{}", (String::from("Status: INCOMPLETE")).rosy_display());
		}
		println!("{}", (String::from("========================")).rosy_display());
		Ok(())
	}
	fn RUN ( counter: &mut f64, global_message: &mut String, operation_complete: &mut bool ) -> Result<()> {
		*counter = 0f64;
		*global_message = String::from("Initial message");
		*operation_complete = false;
		println!("{}", (String::from("Initial state:")).rosy_display());
		DISPLAY_GLOBALS(counter, global_message, operation_complete).with_context(|| format!("...while trying to call procedure DISPLAY_GLOBALS!"))?;
		INCREMENT_COUNTER(counter).with_context(|| format!("...while trying to call procedure INCREMENT_COUNTER!"))?;
		INCREMENT_COUNTER(counter).with_context(|| format!("...while trying to call procedure INCREMENT_COUNTER!"))?;
		INCREMENT_COUNTER(counter).with_context(|| format!("...while trying to call procedure INCREMENT_COUNTER!"))?;
		SET_MESSAGE(&String::from("Hello from global variables!"), global_message).with_context(|| format!("...while trying to call procedure SET_MESSAGE!"))?;
		MARK_COMPLETE(operation_complete).with_context(|| format!("...while trying to call procedure MARK_COMPLETE!"))?;
		println!("{}", (String::from("Final state:")).rosy_display());
		DISPLAY_GLOBALS(counter, global_message, operation_complete).with_context(|| format!("...while trying to call procedure DISPLAY_GLOBALS!"))?;
		if *operation_complete {
			println!("{}", (String::from("Operation is complete!")).rosy_display());
		}
		Ok(())
	}
	RUN(&mut counter, &mut global_message, &mut operation_complete).with_context(|| format!("...while trying to call procedure RUN!"))?;
	// <INJECT_END>
    
    Ok(())
}