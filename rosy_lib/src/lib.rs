mod operators;
mod intrinsics;

pub use operators::*;
pub use intrinsics::*;

pub type RE = f64;
pub type ST = String;
pub type LO = bool;
pub type CM = (f64, f64);
pub type VE = Vec<f64>;

pub enum RosyType {
    RE,
    ST,
    LO,
    CE,
    VE,
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 */