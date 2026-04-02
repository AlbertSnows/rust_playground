pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_equals_4() {
        let mut result = solve_n_queens(4);
        result.sort();
        let mut expected = vec![
            vec![".Q..","...Q","Q...","..Q."],
            vec!["..Q.","Q...","...Q",".Q.."],
        ];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn n_equals_1() {
        assert_eq!(solve_n_queens(1), vec![vec!["Q".to_string()]]);
    }
}
