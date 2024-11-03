use num::Float;

use crate::{subtract, sum_of_squares};

pub fn squared_distance<T: Float>(v: &Vec<T>, w: &Vec<T>) -> Option<T> {
    match subtract(v, w) {
        Some(sub) => sum_of_squares(&sub),
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_squared_distance() {
        assert_eq!(squared_distance(&vec![], &vec![-3., 2.]), None);
        assert_eq!(squared_distance(&vec![1., 2.], &vec![4.]), None);

        assert_eq!(squared_distance(&vec![1., 2.], &vec![1., 2.]), Some(0.));
        assert_eq!(squared_distance(&vec![0., 0.], &vec![3., 4.]), Some(25.));
    }
}
