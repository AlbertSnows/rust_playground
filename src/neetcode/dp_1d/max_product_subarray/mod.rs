pub fn max_product(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    }

    #[test]
    fn with_zero() {
        assert_eq!(max_product(vec![-2, 0, -1]), 0);
    }
}
