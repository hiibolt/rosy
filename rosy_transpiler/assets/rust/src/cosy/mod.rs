mod add;
mod concat;

pub use concat::Concat;

type RE = f64;
type ST = String;
type LO = bool;
type CE = (f64, f64);
type VE = Vec<f64>;

#[derive(Clone)]
pub enum Cosy {
    Real(RE),
    String(ST),
    Logical(LO),
    Complex(CE),
    Vector(VE)
}
impl Cosy {
    fn r#type ( &self ) -> &'static str {
        match self {
            Cosy::Real(_) => "RE (real)",
            Cosy::String(_) => "ST (string)",
            Cosy::Logical(_) => "LO (logical)",
            Cosy::Complex(_) => "CM (complex)",
            Cosy::Vector(_) => "VE (vector)"
        }
    }
    pub fn into_complex ( self ) -> Self {
        match self {
            Cosy::Real(r) => Cosy::Complex((r, 0.0)),
            Cosy::Complex(c) => self,
            Cosy::Vector(v) => {
                assert!(v.len() == 2, "Cannot convert vector of length {} to CM (complex), must have exactly 2 elements!", v.len());

                Cosy::Complex((v[0], v[1]))
            },
            _ => panic!("Fundamentally impossible to convert {} to CM (complex)!", self.r#type())
        }
    }
    pub fn into_usize ( self ) -> usize {
        match self {
            Cosy::Real(r) => {
                assert!(r.fract() == 0.0, "Cannot convert non-integer RE (real) value {} to usize!", r);
                assert!(r >= 0.0, "Cannot convert negative RE (real) value {} to usize!", r);

                r as usize
            },
            _ => panic!("Fundamentally impossible to convert {} to usize!", self.r#type())
        }
    }
    pub fn from_stdin () -> Self {
        use std::io::{self, Write};

        io::stdout().flush().expect("Failed to flush stdout!");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line from stdin!");

        let input = input.trim();

        if let Ok(r) = input.parse::<f64>() {
            Cosy::Real(r)
        } else if let Ok(b) = input.parse::<bool>() {
            Cosy::Logical(b)
        } else if input.starts_with('"') && input.ends_with('"') && input.len() >= 2 {
            Cosy::String(input[1..input.len()-1].to_string())
        } else {
            panic!("Failed to parse input: {}", input);
        }
    }
}
impl std::fmt::Display for Cosy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cosy::Real(n) => write!(f, "{}", n),
            Cosy::String(s) => write!(f, "{}", s),
            Cosy::Logical(b) => write!(f, "{}", b),
            Cosy::Complex((re, im)) => {
                if *im >= 0.0 {
                    write!(f, "({} + {}i)", re, im)
                } else {
                    write!(f, "({} - {}i)", re, -im)
                }
            }
            Cosy::Vector(v) => {
                write!(f, "{{ ")?;
                for (i, elem) in v.iter().enumerate() {
                    write!(f, "{}", elem)?;
                    if i < v.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, " }}")
            }
        }
    }
}
