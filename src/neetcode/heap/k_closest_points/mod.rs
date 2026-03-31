pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let result = k_closest(vec![vec![1, 3], vec![-2, 2]], 1);
        assert_eq!(result, vec![vec![-2, 2]]);
    }

    #[test]
    fn two_closest() {
        let mut result = k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2);
        result.sort();
        let mut expected = vec![vec![3, 3], vec![-2, 4]];
        expected.sort();
        assert_eq!(result, expected);
    }
}
