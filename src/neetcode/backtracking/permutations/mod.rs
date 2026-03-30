pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut result = permute(vec![1, 2, 3]);
        result.sort();
        let mut expected = vec![
            vec![1,2,3], vec![1,3,2], vec![2,1,3],
            vec![2,3,1], vec![3,1,2], vec![3,2,1],
        ];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn single_element() {
        assert_eq!(permute(vec![0]), vec![vec![0]]);
    }
}
