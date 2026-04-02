pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E'],
        ];
        assert_eq!(exist(board, "ABCCED".to_string()), true);
    }

    #[test]
    fn not_found() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E'],
        ];
        assert_eq!(exist(board, "ABCB".to_string()), false);
    }
}
