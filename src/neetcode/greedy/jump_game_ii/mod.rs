pub fn jump(nums: Vec<i32>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn another() {
        assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
