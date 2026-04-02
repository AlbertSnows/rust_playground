pub fn max_profit_cooldown(prices: Vec<i32>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(max_profit_cooldown(vec![1, 2, 3, 0, 2]), 3);
    }

    #[test]
    fn single_price() {
        assert_eq!(max_profit_cooldown(vec![1]), 0);
    }
}
