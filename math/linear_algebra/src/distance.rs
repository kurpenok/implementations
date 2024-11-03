use num::Float;

use crate::{magnitude::magnitude, subtract::subtract};

pub fn distance<T: Float>(v: &Vec<T>, w: &Vec<T>) -> Option<T> {
    match subtract(v, w) {
        Some(sub) => magnitude(&sub),
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_distance() {
        assert_eq!(distance(&vec![], &vec![-3., 2.]), None);
        assert_eq!(distance(&vec![1., 2.], &vec![4.]), None);

        assert_eq!(distance(&vec![1., 2.], &vec![1., 2.]), Some(0.));
        assert_eq!(distance(&vec![0., 0.], &vec![3., 4.]), Some(5.));
    }
}
