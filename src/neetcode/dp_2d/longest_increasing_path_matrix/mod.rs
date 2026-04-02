pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let matrix = vec![vec![9,9,4],vec![6,6,8],vec![2,1,1]];
        assert_eq!(longest_increasing_path(matrix), 4);
    }

    #[test]
    fn single_cell() {
        assert_eq!(longest_increasing_path(vec![vec![1]]), 1);
    }
}
