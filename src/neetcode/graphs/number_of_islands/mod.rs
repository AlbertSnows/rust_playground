pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let grid = vec![
            vec!['1','1','1','1','0'],
            vec!['1','1','0','1','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','0','0','0'],
        ];
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn multiple_islands() {
        let grid = vec![
            vec!['1','1','0','0','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','1','0','0'],
            vec!['0','0','0','1','1'],
        ];
        assert_eq!(num_islands(grid), 3);
    }
}
