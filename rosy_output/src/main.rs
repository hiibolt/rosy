#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use rosy_lib::*;
use anyhow::{Result, Context};

fn run () -> Result<()> {
    // <INJECT_START>
	// <INJECT_END>

    Ok(())
}
fn main () -> Result<()> {
    run()
        .context("Encountered a runtime error")?;

    Ok(())
}