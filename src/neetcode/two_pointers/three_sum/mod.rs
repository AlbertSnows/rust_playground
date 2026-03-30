pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        result.sort();
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn no_solution() {
        assert_eq!(three_sum(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>);
    }
}
