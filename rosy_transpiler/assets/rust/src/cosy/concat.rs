use super::Cosy;

pub trait Concat<T> {
    type Output;
    fn concat(self, other: T) -> Self::Output;
}

impl Concat<&Cosy> for &Cosy {
    type Output = Cosy;
    fn concat(self, other: &Cosy) -> Self::Output {
        match (self, other) {
            (Cosy::Real(a), Cosy::Real(b)) => Cosy::Vector(vec![*a, *b]),
            (Cosy::Real(a), Cosy::Vector(b)) => {
                let mut b = b.clone();
                b.insert(0, *a);
                Cosy::Vector(b)
            },
            (Cosy::String(a), Cosy::String(b)) => Cosy::String(a.clone() + b),
            (Cosy::Vector(a), Cosy::Real(b)) => {
                let mut a = a.clone();
                a.push(*b);
                Cosy::Vector(a)
            },
            (Cosy::Vector(a), Cosy::Vector(b)) => {
                let mut a = a.clone();
                a.extend(b);
                Cosy::Vector(a)
            },
            
            // Fallback
            (a, b) => panic!(
                "Concat not supported between types {} and {}",
                a.r#type(),
                b.r#type()
            ),
        }
    }
}