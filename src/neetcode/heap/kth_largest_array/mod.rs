pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn duplicates() {
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }
}
