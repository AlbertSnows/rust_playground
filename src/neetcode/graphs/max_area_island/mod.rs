pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let grid = vec![
            vec![0,0,1,0,0,0,0,1,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
            vec![0,1,1,0,1,0,0,0,0,0,0,0,0],
            vec![0,1,0,0,1,1,0,0,1,0,1,0,0],
            vec![0,1,0,0,1,1,0,0,1,1,1,0,0],
            vec![0,0,0,0,0,0,0,0,0,0,1,0,0],
            vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
            vec![0,0,0,0,0,0,0,1,1,0,0,0,0],
        ];
        assert_eq!(max_area_of_island(grid), 6);
    }

    #[test]
    fn all_water() {
        assert_eq!(max_area_of_island(vec![vec![0, 0, 0]]), 0);
    }
}
