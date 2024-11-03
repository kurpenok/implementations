use num::Float;

pub fn add<T: Float>(v: &[T], w: &[T]) -> Option<Vec<T>> {
    if v.len() != w.len() {
        return None;
    }

    Some(v.iter().zip(w.iter()).map(|(&x, &y)| x + y).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_addition() {
        assert_eq!(add(&vec![], &vec![-3., 2.]), None);
        assert_eq!(add(&vec![1., 2.], &vec![4.]), None);

        assert_eq!(add(&vec![1., 2.], &vec![4., 5.]), Some(vec![5., 7.]));
        assert_eq!(add(&vec![3., -2.], &vec![-3., 2.]), Some(vec![0., 0.]));
    }
}
