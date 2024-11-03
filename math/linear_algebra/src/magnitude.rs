use num::Float;

use crate::sum_of_squares::sum_of_squares;

pub fn magnitude<T: Float>(v: &Vec<T>) -> T {
    sum_of_squares(v).unwrap().sqrt()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_magnitude() {
        assert_eq!(magnitude(&vec![3., 4.]), 5.);
    }
}
