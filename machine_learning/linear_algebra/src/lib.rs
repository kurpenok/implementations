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
}
