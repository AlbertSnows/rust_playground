pub fn max_profit(prices: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn no_profit() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
