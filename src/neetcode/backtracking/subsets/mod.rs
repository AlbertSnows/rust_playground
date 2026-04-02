pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut result = subsets(vec![1, 2, 3]);
        result.sort();
        let mut expected = vec![vec![], vec![1], vec![2], vec![3], vec![1,2], vec![1,3], vec![2,3], vec![1,2,3]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn single_element() {
        let mut result = subsets(vec![0]);
        result.sort();
        assert_eq!(result, vec![vec![], vec![0]]);
    }
}
