pub fn find_min(nums: Vec<i32>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn not_rotated() {
        assert_eq!(find_min(vec![1, 2, 3]), 1);
    }
}
