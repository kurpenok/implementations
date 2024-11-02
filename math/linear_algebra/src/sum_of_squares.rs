use num::Num;

use crate::dot::dot;

pub fn sum_of_squares<T>(v: &Vec<T>) -> Option<T>
where
    T: Copy + Num,
{
    dot(v, v)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_sum_of_squares() {
        assert_eq!(sum_of_squares(&vec![1, 2]), Some(5));
        assert_eq!(sum_of_squares(&vec![3, -2]), Some(13));

        assert_eq!(sum_of_squares(&vec![1., 2.]), Some(5.));
        assert_eq!(sum_of_squares(&vec![3., -2.]), Some(13.));
    }
}
