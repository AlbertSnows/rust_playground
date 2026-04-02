pub fn can_jump(nums: Vec<i32>) -> bool {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible() {
        assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn impossible() {
        assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
