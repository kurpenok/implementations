use num::Float;

pub fn subtract<T: Float>(v: &[T], w: &[T]) -> Option<Vec<T>> {
    if v.len() != w.len() {
        return None;
    }

    Some(v.iter().zip(w.iter()).map(|(&x, &y)| x - y).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_subtraction() {
        assert_eq!(subtract(&vec![], &vec![-3., 2.]), None);
        assert_eq!(subtract(&vec![1., 2.], &vec![4.]), None);

        assert_eq!(subtract(&vec![1., 2.], &vec![4., 5.]), Some(vec![-3., -3.]));
        assert_eq!(subtract(&vec![3., 2.], &vec![-3., 2.]), Some(vec![6., 0.]));
    }
}
