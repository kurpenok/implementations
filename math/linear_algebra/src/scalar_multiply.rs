use num::Num;

pub fn scalar_multiply<T>(c: T, v: &[T]) -> Vec<T>
where
    T: Copy + Num,
{
    v.iter().map(|&n| c * n).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_scalar_multiply() {
        assert_eq!(scalar_multiply(1, &vec![1, 2, 3]), [1, 2, 3]);
        assert_eq!(scalar_multiply(2., &vec![1., 2., 3.]), [2., 4., 6.]);
    }
}
