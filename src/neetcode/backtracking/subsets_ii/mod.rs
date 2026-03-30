pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut result = subsets_with_dup(vec![1, 2, 2]);
        result.sort();
        result.dedup();
        let mut expected = vec![vec![], vec![1], vec![1,2], vec![1,2,2], vec![2], vec![2,2]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn all_same() {
        let mut result = subsets_with_dup(vec![0]);
        result.sort();
        assert_eq!(result, vec![vec![], vec![0]]);
    }
}
