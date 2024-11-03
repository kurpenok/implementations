use num::Float;

use crate::sum_of_squares;

pub fn magnitude<T: Float>(v: &Vec<T>) -> Option<T> {
    match sum_of_squares(v) {
        Some(sum) => Some(sum.sqrt()),
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_magnitude() {
        assert_eq!(magnitude(&vec![3., 4.]), Some(5.));
        assert_eq!(magnitude(&vec![6., 8.]), Some(10.));
    }
}
