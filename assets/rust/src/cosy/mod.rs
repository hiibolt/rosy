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
