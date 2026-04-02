pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let grid = vec![vec![2,1,1],vec![1,1,0],vec![0,1,1]];
        assert_eq!(oranges_rotting(grid), 4);
    }

    #[test]
    fn impossible() {
        let grid = vec![vec![2,1,1],vec![0,1,1],vec![1,0,1]];
        assert_eq!(oranges_rotting(grid), -1);
    }
}
