pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn not_found() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
