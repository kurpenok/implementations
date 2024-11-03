use num::Float;

pub fn scalar_multiply<T: Float>(c: T, v: &[T]) -> Vec<T> {
    v.iter().map(|&n| c * n).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_scalar_multiply() {
        assert_eq!(scalar_multiply(2., &vec![1., 2., 3.]), [2., 4., 6.]);
        assert_eq!(scalar_multiply(4., &vec![2., 4., 8.]), [8., 16., 32.]);
    }
}
