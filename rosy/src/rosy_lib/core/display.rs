use crate::rosy_lib::{RE, ST, LO, CM, VE, DA, CD};

pub trait RosyDisplay {
    fn rosy_display(self) -> String;
}
impl RosyDisplay for &RE {
    fn rosy_display(self) -> String {
        format!(" {}{:17.15}     ", if self.is_sign_negative() {"-"} else {" "}, self.abs() )
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
            " ({}     ,{}     )", 
            self.0.rosy_display()
                .chars()
                .take(12)
                .collect::<String>(),
            self.1.rosy_display()
                .chars()
                .take(12)
                .collect::<String>()
        )
    }
}

impl RosyDisplay for &VE {
    fn rosy_display(self) -> String {
        let elements: Vec<String> = self.iter()
            .map(|x| format!(" {}", x.rosy_display().chars().take(10).collect::<String>()))
            .collect();
        format!("{}", elements.join("    ") )
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
                        if i > 0 && i % 2 == 0 {
                            acc.push_str(" "); // Double space between variable pairs
                        }
                        acc.push_str(&format!("{}", exp));
                        if i < exps.len() - 1 {
                            acc.push(' ');
                        }
                        acc
                    })

            };
            output.push_str(&format!("{}  {:18.15}       {}   {}\n", 
                idx + 1, coeff, order, exp_str));
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
                        if i > 0 && i % 2 == 0 {
                            acc.push_str(" "); // Double space between variable pairs
                        }
                        acc.push_str(&format!("{}", exp));
                        if i < exps.len() - 1 {
                            acc.push(' ');
                        }
                        acc
                    })

            };
            output.push_str(&format!("     {} {:18.15}     {:18.15}       {}   {}\n",
                idx + 1, real_coeff, imag_coeff, order, exp_str));
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