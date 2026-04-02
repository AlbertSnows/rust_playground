pub fn trap(height: Vec<i32>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn another_case() {
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
