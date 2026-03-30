pub fn solve(board: &mut Vec<Vec<char>>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut board = vec![
            vec!['X','X','X','X'],
            vec!['X','O','O','X'],
            vec!['X','X','O','X'],
            vec!['X','O','X','X'],
        ];
        solve(&mut board);
        assert_eq!(board, vec![
            vec!['X','X','X','X'],
            vec!['X','X','X','X'],
            vec!['X','X','X','X'],
            vec!['X','O','X','X'],
        ]);
    }

    #[test]
    fn no_surrounded() {
        let mut board = vec![vec!['X']];
        solve(&mut board);
        assert_eq!(board, vec![vec!['X']]);
    }
}
