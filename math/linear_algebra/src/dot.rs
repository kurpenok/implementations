use num::Float;

pub fn dot<T: Float>(v: &Vec<T>, w: &Vec<T>) -> Option<T> {
    if v.len() != w.len() {
        return None;
    }

    Some(
        v.iter()
            .zip(w.iter())
            .map(|(&x, &y)| x * y)
            .fold(T::zero(), |result, x| result + x),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_dot() {
        assert_eq!(dot(&vec![], &vec![-3., 2.]), None);
        assert_eq!(dot(&vec![1., 2.], &vec![4.]), None);

        assert_eq!(dot(&vec![1., 2.], &vec![4., 5.]), Some(14.));
        assert_eq!(dot(&vec![3., -2.], &vec![-3., 2.]), Some(-13.));
    }
}
