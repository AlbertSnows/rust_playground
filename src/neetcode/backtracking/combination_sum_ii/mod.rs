pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut result = combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
        result.sort();
        let mut expected = vec![vec![1,1,6], vec![1,2,5], vec![1,7], vec![2,6]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn no_solution() {
        assert_eq!(combination_sum2(vec![2], 1), vec![] as Vec<Vec<i32>>);
    }
}
