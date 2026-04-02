pub fn single_number(nums: Vec<i32>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn longer() {
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }
}
