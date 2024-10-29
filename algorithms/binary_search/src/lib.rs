pub fn binary_search<T: Ord>(array: &[T], target: T) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let middle = left + (right - left) / 2;
        if array[middle] == target {
            return Some(middle);
        } else if array[middle] < target {
            left = middle + 1;
        } else if array[middle] > target {
            right = middle;
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(&[2, 4, 7, 10, 27, 52], 2), Some(0));
        assert_eq!(binary_search(&[2, 4, 7, 10, 27, 52], 10), Some(3));
        assert_eq!(binary_search(&[2, 4, 7, 10, 27, 52], 52), Some(5));
        assert_eq!(binary_search(&[2, 4, 7, 10, 27, 52], 0), None);

        assert_eq!(binary_search(&vec![2, 4, 7, 10, 27, 52], 2), Some(0));
        assert_eq!(binary_search(&vec![2, 4, 7, 10, 27, 52], 10), Some(3));
        assert_eq!(binary_search(&vec![2, 4, 7, 10, 27, 52], 52), Some(5));
        assert_eq!(binary_search(&vec![2, 4, 7, 10, 27, 52], 0), None);
    }
}
