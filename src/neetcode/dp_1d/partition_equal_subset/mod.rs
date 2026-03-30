pub fn can_partition(nums: Vec<i32>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible() {
        assert_eq!(can_partition(vec![1, 5, 11, 5]), true);
    }

    #[test]
    fn impossible() {
        assert_eq!(can_partition(vec![1, 2, 3, 5]), false);
    }
}
