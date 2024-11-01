pub type Vector = Vec<f64>;

pub fn add(v: &Vector, w: &Vector) -> Option<Vector> {
    if v.len() != w.len() {
        return None;
    }

    let mut sum: Vec<f64> = Vec::new();
    for i in 0..v.len() {
        sum.push(v[i] + w[i]);
    }

    Some(sum)
}

pub fn subtract(v: &Vector, w: &Vector) -> Option<Vector> {
    if v.len() != w.len() {
        return None;
    }

    let mut sub: Vec<f64> = Vec::new();
    for i in 0..v.len() {
        sub.push(v[i] - w[i]);
    }

    Some(sub)
}

pub fn vector_sum(vectors: &Vec<&Vector>) -> Option<Vec<f64>> {
    if vectors.iter().all(|v| v.len() == vectors[0].len()) {
        let mut corresponding_sum: Vec<f64> = Vec::new();
        for i in 0..vectors[0].len() {
            let mut sum: f64 = 0.;
            for v in vectors {
                sum += v[i];
            }
            corresponding_sum.push(sum);
        }

        return Some(corresponding_sum);
    }

    None
}

pub fn scalar_multiply(c: f64, v: &Vector) -> Vector {
    let mut w: Vec<f64> = Vec::new();
    for i in 0..v.len() {
        w.push(c * v[i]);
    }
    w
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_addition() {
        assert_eq!(add(&vec![], &vec![-3., 2.]), None);
        assert_eq!(add(&vec![1., 2.], &vec![]), None);

        assert_eq!(add(&vec![3., -2.], &vec![-3., 2.]), Some(vec![0., 0.]));
        assert_eq!(add(&vec![1., 2.], &vec![4., 5.]), Some(vec![5., 7.]));
    }

    #[test]
    fn test_vector_subtraction() {
        assert_eq!(subtract(&vec![], &vec![-3., 2.]), None);
        assert_eq!(subtract(&vec![1., 2.], &vec![]), None);

        assert_eq!(subtract(&vec![3.], &vec![-3.]), Some(vec![6.]));
        assert_eq!(subtract(&vec![1.], &vec![4.]), Some(vec![-3.]));
    }

    #[test]
    fn test_vector_corresponding_sum() {
        assert_eq!(vector_sum(&vec![&vec![1., 2.], &vec![3.]]), None);
        assert_eq!(vector_sum(&vec![&vec![4.], &vec![]]), None);

        assert_eq!(vector_sum(&vec![&vec![1.], &vec![3.]]), Some(vec![4.]));
        assert_eq!(vector_sum(&vec![&vec![3.], &vec![6.]]), Some(vec![9.]));
    }
}
