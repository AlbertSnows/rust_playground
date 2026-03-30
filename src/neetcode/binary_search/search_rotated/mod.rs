pub fn search_rotated(nums: Vec<i32>, target: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(search_rotated(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn not_found() {
        assert_eq!(search_rotated(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }
}
