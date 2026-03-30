pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn single_window() {
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
    }
}
