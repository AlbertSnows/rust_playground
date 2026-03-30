pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut result = combination_sum(vec![2, 3, 6, 7], 7);
        result.sort();
        let mut expected = vec![vec![2, 2, 3], vec![7]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn no_solution() {
        assert_eq!(combination_sum(vec![2], 3), vec![] as Vec<Vec<i32>>);
    }
}
