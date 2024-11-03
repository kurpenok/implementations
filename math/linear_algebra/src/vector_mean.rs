use num::Float;

pub fn vector_mean<T: Float>(vectors: &[&[T]]) -> Option<Vec<T>> {
    if vectors.len() == 0 || !vectors.iter().all(|v| v.len() == vectors[0].len()) {
        return None;
    }

    let mut result: Vec<T> = vec![T::zero(); vectors[0].len()];
    let mut count: T = T::zero();
    for i in 0..vectors[0].len() {
        count = T::zero();
        for v in vectors {
            result[i] = result[i].add(v[i]);
            count = count.add(T::one());
        }
    }

    for i in 0..result.len() {
        result[i] = result[i] / count;
    }

    Some(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_mean() {
        let a = vec![1., 2., 3., 4.];
        let b = vec![2., 4., 6., 8., 10.];
        let vectors: Vec<&[f32]> = vec![&a, &b];
        assert_eq!(vector_mean(&vectors), None);

        let a = vec![2., 2., 4., 4., 6.];
        let b = vec![2., 4., 6., 8., 10.];
        let vectors: Vec<&[f64]> = vec![&a, &b];
        assert_eq!(vector_mean(&vectors), Some(vec![2., 3., 5., 6., 8.]));
    }
}
