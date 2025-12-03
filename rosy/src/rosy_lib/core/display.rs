use crate::rosy_lib::{RE, ST, LO, CM, VE, DA, CD};

fn sci(x: f64) -> (f64, i32) {
    if x == 0.0 {
        return (0.0, 0);
    }

    if x >= 1.0 {
        // No exponent shifting needed for your rules.
        (x, 0)
    } else {
        let exp = (-x.log10()).floor() as i32;
        let base = x * 10f64.powi(exp);
        (base, -exp)
    }
}
fn display_re (
    num: RE,
    precision: usize,
    exponent_precision: usize,
    spaces: usize
) -> String {
    if num.abs() < 1f64 && num != 0f64 {
        let (mantissa, exponent) = sci(num.abs());

        if num.is_sign_positive() {
            format!(
                "0.{}{}",
                format!("{:.precision$}", mantissa, precision=precision)
                    .chars()
                    .skip(2) // Skip "0."
                    .take(precision)
                    .collect::<String>(),
                if exponent != 0 {
                    format!(
                        "E{:+0exponent_precision$}",
                        exponent,
                        exponent_precision=exponent_precision
                    )
                } else {
                    " ".repeat(spaces)
                }
            )
        } else {
            format!(
                "-.{}{}",
                format!("{:.precision$}", mantissa, precision=precision)
                    .chars()
                    .skip(2) // Skip "0."
                    .take(precision)
                    .collect::<String>(),
                if exponent != 0 {
                    format!(
                        "E{:+0exponent_precision$}",
                        exponent,
                        exponent_precision=exponent_precision
                    )
                } else {
                    " ".repeat(spaces)
                }
            )
        }
    } else {
        format!(
            "{}{}{}",
            if num.is_sign_negative() {"-"} else {" "},
            format!(
                "{:.precision$}",
                num.abs(),
                precision=precision
            ).chars().take(precision + 1).collect::<String>(),
            " ".repeat(spaces),
        )
    }
}
pub trait RosyDisplay {
    fn rosy_display(self) -> String;
}
impl RosyDisplay for &RE {
    fn rosy_display(self) -> String {
        display_re(*self, 16, 3, 4)
    }
}

impl RosyDisplay for &ST {
    fn rosy_display(self) -> String {
        self.to_string()
    }
}

impl RosyDisplay for &LO {
    fn rosy_display(self) -> String {
        if *self { "TRUE" } else { "FALSE" }.to_string()
    }
}

impl RosyDisplay for &CM {
    fn rosy_display(self) -> String {
        // COSY format: (  real     ,  imag     )
        format!(
            " ( {}, {})", 
            display_re(self.0, 9, 4, 5),
            display_re(self.1, 9, 4, 5)
        )
    }
}

impl RosyDisplay for &VE {
    fn rosy_display(self) -> String {
        let elements: Vec<String> = self.iter()
            .map(|x| format!(" {}", x.rosy_display().chars().take(9).collect::<String>()))
            .collect();
        format!("{}", elements.join("     ") )
    }
}

impl RosyDisplay for &DA {
    fn rosy_display(self) -> String {
        // Output in COSY format: multi-line with all coefficients
        
        // Get all coefficients
        let coeffs: Vec<_> = self.coeffs_iter().collect();
        if coeffs.is_empty() {
            return "     I  COEFFICIENT            ORDER EXPONENTS\n     1   0.000000000000000       0   0 0\n     -----------------------------------".to_string();
        }
        
        // Sort by graded reverse lexicographic order (COSY format)
        // First by total order, then by exponents in reverse lexicographic order
        let mut sorted = coeffs.clone();
        sorted.sort_by(|(m1, _), (m2, _)| {
            m1.total_order.cmp(&m2.total_order)
                .then_with(|| {
                    // Reverse lexicographic: compare from right to left
                    for i in (0..m1.exponents.len()).rev() {
                        match m1.exponents[i].cmp(&m2.exponents[i]) {
                            std::cmp::Ordering::Equal => continue,
                            ord => return ord,
                        }
                    }
                    std::cmp::Ordering::Equal
                })
        });
        
        let mut output = String::new();
        output.push_str("I  COEFFICIENT            ORDER EXPONENTS\n");
        for (idx, (monomial, coeff)) in sorted.iter().enumerate() {
            let order = monomial.total_order;
            let exp_str = {
                // For 6 exponents, should match: '1 0  1 0  0 0'
                let exps = &monomial.exponents;

                exps.iter()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i, exp)| {
                        if i % 2 == 0 {
                            acc.push_str(&format!("{}", exp));
                        } else {
                            acc.push_str(&format!("{:>2}  ", exp));
                        }
                        acc
                    })
            };
            output.push_str(&format!(
                "{}  {} {}   {}\n", 
                idx + 1,
                coeff.rosy_display(),
                format!("{:>3}", order),
                exp_str.trim_end()
            ));
        }

        let last_line_length = output.lines().last().unwrap_or("").len();
        output.push_str(&"-".repeat(last_line_length));
        output.lines()
            .map(|st| format!("     {}", st))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl RosyDisplay for &CD {
    fn rosy_display(self) -> String {
        // Output in COSY format: multi-line with all complex coefficients
        
        // Get real and imaginary parts
        let real_part = self.real_part();
        let imag_part = self.imag_part();
        
        // Combine all monomials from both parts
        let mut all_monomials = std::collections::HashSet::new();
        for (m, _) in real_part.coeffs_iter() {
            all_monomials.insert(*m);
        }
        for (m, _) in imag_part.coeffs_iter() {
            all_monomials.insert(*m);
        }
        
        if all_monomials.is_empty() {
            return "     I  COEFFICIENTS                           ORDER EXPONENTS\n     1  0.000000000000000      0.000000000000000       0   0 0\n                                      ".to_string();
        }
        
        // Sort by graded reverse lexicographic order (COSY format)
        // First by total order, then by exponents in reverse lexicographic order
        let mut sorted: Vec<_> = all_monomials.into_iter().collect();
        sorted.sort_by(|m1, m2| {
            m1.total_order.cmp(&m2.total_order)
                .then_with(|| {
                    // Reverse lexicographic: compare from right to left
                    for i in (0..m1.exponents.len()).rev() {
                        match m1.exponents[i].cmp(&m2.exponents[i]) {
                            std::cmp::Ordering::Equal => continue,
                            ord => return ord,
                        }
                    }
                    std::cmp::Ordering::Equal
                })
        });
        
        let mut output = String::new();
        output.push_str("     I  COEFFICIENTS                           ORDER EXPONENTS\n");
        for (idx, monomial) in sorted.iter().enumerate() {
            let real_coeff = real_part.get_coeff(monomial);
            let imag_coeff = imag_part.get_coeff(monomial);
            let order = monomial.total_order;
            let exp_str = {
                // For 6 exponents, should match: '1 0  1 0  0 0'
                let exps = &monomial.exponents;

                exps.iter()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i, exp)| {
                        if i % 2 == 0 {
                            acc.push_str(&format!("{:>2}", exp));
                        } else {
                            acc.push_str(&format!("{:>2} ", exp));
                        }
                        acc
                    })
            };
            output.push_str(&format!(
                "     {} {} {} {:>3}  {}\n",
                idx + 1, 
                real_coeff.rosy_display(), 
                imag_coeff.rosy_display(),
                order,
                exp_str.trim_end()
            ));
        }
        output.push_str("                                      ");
        output
    }
}

// Required as loops cast to `usize`
impl RosyDisplay for &usize {
    fn rosy_display(self) -> String {
        self.to_string()
    }
}

impl RosyDisplay for &str {
    fn rosy_display(self) -> String {
        self.to_string()
    }
}