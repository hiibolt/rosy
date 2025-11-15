use crate::rosy_lib::{RE, ST, LO, CM, VE, DA, CD};

pub trait RosyDisplay {
    fn rosy_display(self) -> String;
}
impl RosyDisplay for &RE {
    fn rosy_display(self) -> String {
        format!(" {:18.15}     ", self)
    }
}

impl RosyDisplay for &ST {
    fn rosy_display(self) -> String {
        self.to_string()
    }
}

impl RosyDisplay for &LO {
    fn rosy_display(self) -> String {
        // COSY outputs 1.0 for false, 2.0 for true
        let val = if *self { 2.0 } else { 1.0 };
        format!(" {:18.15}     ", val)
    }
}

impl RosyDisplay for &CM {
    fn rosy_display(self) -> String {
        // COSY format: (  real     ,  imag     )
        format!(" (  {:.8}     ,  {:.8}     )", self.0, self.1)
    }
}

impl RosyDisplay for &VE {
    fn rosy_display(self) -> String {
        // COSY only outputs the first element
        if let Some(first) = self.first() {
            format!(" {:18.15}     ", first)
        } else {
            " 0".to_string()
        }
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
                    // Reverse lexicographic: reverse the normal comparison
                    m2.exponents.cmp(&m1.exponents)
                })
        });
        
        let mut output = String::new();
        output.push_str("     I  COEFFICIENT            ORDER EXPONENTS\n");
        for (idx, (monomial, coeff)) in sorted.iter().enumerate() {
            let order = monomial.total_order;
            let exps = &monomial.exponents;
            output.push_str(&format!("     {}  {:18.15}       {}   {} {}\n", 
                idx + 1, coeff, order, exps[0], exps[1]));
        }
        output.push_str("     -----------------------------------");
        output
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
                    // Reverse lexicographic: reverse the normal comparison
                    m2.exponents.cmp(&m1.exponents)
                })
        });
        
        let mut output = String::new();
        output.push_str("     I  COEFFICIENTS                           ORDER EXPONENTS\n");
        for (idx, monomial) in sorted.iter().enumerate() {
            let real_coeff = real_part.get_coeff(monomial);
            let imag_coeff = imag_part.get_coeff(monomial);
            let order = monomial.total_order;
            let exps = &monomial.exponents;
            output.push_str(&format!("     {} {:18.15}     {:18.15}       {}   {} {}\n",
                idx + 1, real_coeff, imag_coeff, order, exps[0], exps[1]));
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