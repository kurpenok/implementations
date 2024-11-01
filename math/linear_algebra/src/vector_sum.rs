use num::Num;

pub fn vector_sum<T>(vectors: &[&[T]]) -> Option<Vec<T>>
where
    T: Copy + Num,
{
    if vectors.len() == 0 || !vectors.iter().all(|v| v.len() == vectors[0].len()) {
        return None;
    }

    let mut result = vec![T::zero(); vectors[0].len()];
    for v in vectors {
        for (i, &value) in v.iter().enumerate() {
            result[i] = result[i] + value;
        }
    }

    Some(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_sum() {
        let a = vec![1, 2, 3, 4];
        let b = vec![2, 4, 6, 8, 10];
        let vectors: Vec<&[i32]> = vec![&a, &b];
        assert_eq!(vector_sum(&vectors), None);

        let a = vec![1, 2, 3, 4, 5];
        let b = vec![2, 4, 6, 8, 10];
        let vectors: Vec<&[i32]> = vec![&a, &b];
        assert_eq!(vector_sum(&vectors), Some(vec![3, 6, 9, 12, 15]));

        let a = vec![1., 2., 3., 4., 5.];
        let b = vec![2., 4., 6., 8., 10.];
        let vectors: Vec<&[f64]> = vec![&a, &b];
        assert_eq!(vector_sum(&vectors), Some(vec![3., 6., 9., 12., 15.]));
    }
}
