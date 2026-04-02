pub fn max_coins(nums: Vec<i32>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(max_coins(vec![3, 1, 5, 8]), 167);
    }

    #[test]
    fn single() {
        assert_eq!(max_coins(vec![1, 5]), 10);
    }
}
