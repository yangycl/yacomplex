pub mod functions;
pub mod types;

use crate::types::types::Complex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Complex::new(2.0, 2.5);
        let a:Complex = Complex::new(3.5, 2.7);
        let b = Complex::new(4.1, 1.8);
        assert_eq!(result.r, 2.0);
        assert_eq!(result.i, 2.5);
        assert_eq!(Complex::add(a, b), Complex::new(7.6, 4.5));
    }
}
