//! DAPRV and DAREV - DA vector print and read routines.
//!
//! DAPRV writes an array of DA vectors in COSY-format tabular output.
//! DAREV reads an array of DA vectors back from that format.

use anyhow::{Result, Context, bail};

use crate::rosy_lib::taylor::{DA, get_config};
use crate::rosy_lib::taylor::da::DACoefficient;
use crate::rosy_lib::taylor::Monomial;
use crate::rosy_lib::core::display::RosyDisplay;

/// Write an array of DA vectors in COSY INFINITY DAPRV format.
///
/// Arguments:
/// - `array`: the DA vector array (`Vec<DA>`)
/// - `num_components`: number of components to print
/// - `max_vars`: maximum number of variables in the expansion
/// - `current_vars`: current number of main variables
/// - `unit`: output unit number (6 = stdout, otherwise file unit)
pub fn rosy_daprv(
    array: &Vec<DA>,
    num_components: usize,
    _max_vars: usize,
    current_vars: usize,
    unit: u64,
) -> Result<()> {
    let output = format_daprv(array, num_components, _max_vars, current_vars)?;

    if unit == 6 {
        print!("{}", output);
    } else {
        // Write to file
        // Write without the trailing newline that write_to_unit adds
        for line in output.lines() {
            crate::rosy_lib::core::file_io::rosy_write_to_unit(unit, line)?;
        }
    }

    Ok(())
}

/// Format DAPRV output in COSY-compatible format.
fn format_daprv(
    array: &Vec<DA>,
    num_components: usize,
    _max_vars: usize,
    current_vars: usize,
) -> Result<String> {
    let mut output = String::new();

    // Collect all unique monomials from all components
    let mut all_monomials: Vec<Monomial> = Vec::new();
    for i in 0..num_components.min(array.len()) {
        for (m, _) in array[i].coeffs_iter() {
            if !all_monomials.contains(m) {
                all_monomials.push(*m);
            }
        }
    }

    // Sort monomials: first by total order, then by exponents
    all_monomials.sort_by(|m1, m2| {
        m1.total_order.cmp(&m2.total_order)
            .then_with(|| {
                for i in (0..m1.exponents.len()).rev() {
                    match m1.exponents[i].cmp(&m2.exponents[i]) {
                        std::cmp::Ordering::Equal => continue,
                        ord => return ord,
                    }
                }
                std::cmp::Ordering::Equal
            })
    });

    // If no monomials, print a zero entry
    if all_monomials.is_empty() {
        all_monomials.push(Monomial::constant());
    }

    // Print header
    output.push_str(&format!(
        "  I  COEFFICIENT          "));
    for comp in 1..=num_components.min(array.len()) {
        output.push_str(&format!("     {:>2}             ", comp));
    }
    output.push_str("ORDER EXPONENTS\n");

    // Print each monomial row
    for (idx, monomial) in all_monomials.iter().enumerate() {
        let order = monomial.total_order;
        let exp_str = build_exp_str(&monomial.exponents, current_vars);

        output.push_str(&format!("{:>3}  ", idx + 1));

        // Print coefficient for each component
        for i in 0..num_components.min(array.len()) {
            let coeff = array[i].get_coeff(monomial);
            output.push_str(&format!("{} ", coeff.rosy_display()));
        }

        output.push_str(&format!("{:>5}   {}\n", order, exp_str));
    }

    // Print separator
    let sep_len = 30 + num_components.min(array.len()) * 24;
    output.push_str(&"-".repeat(sep_len.min(132)));
    output.push('\n');

    Ok(output)
}

/// Read an array of DA vectors from COSY DAPRV format.
///
/// Arguments:
/// - `array`: the DA vector array to read into
/// - `num_components`: number of components to read
/// - `max_vars`: maximum number of variables
/// - `current_vars`: current number of main variables  
/// - `unit`: input unit number
pub fn rosy_darev(
    array: &mut Vec<DA>,
    num_components: usize,
    _max_vars: usize,
    current_vars: usize,
    unit: u64,
) -> Result<()> {
    // Read lines from the file
    let mut lines = Vec::new();
    
    // Read the header line
    let _header = crate::rosy_lib::core::file_io::rosy_read_from_unit(unit)
        .context("Failed to read header line in DAREV")?;
    
    // Read coefficient lines until we hit the separator
    loop {
        let line = crate::rosy_lib::core::file_io::rosy_read_from_unit(unit)
            .context("Failed to read line in DAREV")?;
        
        // Check if this is a separator line (all dashes)
        if line.trim().chars().all(|c| c == '-') && !line.trim().is_empty() {
            break;
        }
        
        lines.push(line);
    }

    // Ensure array is big enough
    while array.len() < num_components {
        array.push(DA::zero());
    }

    // Zero out the components we're reading into
    for i in 0..num_components.min(array.len()) {
        array[i] = DA::zero();
    }

    // Parse each line
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Parse the line: index, coefficients, order, exponents
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        if tokens.len() < 2 + num_components {
            continue; // Skip malformed lines
        }

        // First token is the index (1-based), skip it
        // Next num_components tokens are coefficients
        // Then order
        // Then exponents
        let mut coeffs = Vec::new();
        for i in 0..num_components {
            if let Ok(coeff) = tokens[1 + i].parse::<f64>() {
                coeffs.push(coeff);
            } else {
                coeffs.push(0.0);
            }
        }

        // Order is after the coefficients
        let order_idx = 1 + num_components;
        if order_idx >= tokens.len() {
            continue;
        }
        
        // Exponents start after order
        let exp_start = order_idx + 1;
        let mut exponents = [0u8; 6];
        for i in 0..current_vars.min(6) {
            if exp_start + i < tokens.len() {
                if let Ok(exp) = tokens[exp_start + i].parse::<u8>() {
                    exponents[i] = exp;
                }
            }
        }

        let monomial = Monomial::new(exponents);

        // Set coefficients for each component
        for (i, &coeff) in coeffs.iter().enumerate() {
            if i < array.len() && coeff.abs() > 1e-15 {
                array[i].set_coeff(monomial, coeff);
            }
        }
    }

    Ok(())
}

/// Build exponent string for DAPRV display.
fn build_exp_str(exponents: &[u8], num_vars: usize) -> String {
    let mut result = String::new();
    for i in 0..num_vars.min(exponents.len()) {
        if i % 2 == 0 {
            result.push_str(&format!("{:>2}", exponents[i]));
        } else {
            result.push_str(&format!("{:>2} ", exponents[i]));
        }
    }
    result.trim_end().to_string()
}
