use std::ops::Add;

use super::Cosy;


fn add_vector_to_vector ( vec_1: &Vec<f64>, vec_2: &Vec<f64> ) -> Vec<f64> {
    assert!(vec_1.len() == vec_2.len(), "Cannot add vectors of different lengths");
    vec_1.into_iter().zip(vec_2).map(|(x, y)| *x + *y).collect()
}
impl Add for &Cosy {
    type Output = Cosy;

    fn add(self, other: &Cosy) -> Self::Output {
        match (self, other) {
            (Cosy::Real(a), Cosy::Real(b)) => Cosy::Real(a + *b),
            (Cosy::Real(a), Cosy::Complex((br, bi))) => Cosy::Complex((*a + *br, *bi)),
            (Cosy::Real(a), Cosy::Vector(b)) => Cosy::Vector(b.into_iter().map(|x| x + a).collect()),
            
            (Cosy::Logical(a), Cosy::Logical(b)) => Cosy::Logical(*a || *b),

            (Cosy::Complex((ar, ai)), Cosy::Real(b)) => Cosy::Complex((*ar + *b, *ai)),
            (Cosy::Complex((ar, ai)), Cosy::Complex((br, bi))) => Cosy::Complex((ar + *br, ai + *bi)),

            (Cosy::Vector(a), Cosy::Real(b)) => Cosy::Vector(a.into_iter().map(|x| x + *b).collect()),
            (Cosy::Vector(a), Cosy::Vector(b)) => Cosy::Vector(add_vector_to_vector(a, b)),

            // Fallback
            (a, b) => panic!(
                "Addition not supported between types {} and {}",
                a.r#type(),
                b.r#type()
            ),
        }
    }
}
