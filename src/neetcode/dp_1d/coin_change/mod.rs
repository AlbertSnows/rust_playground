pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(coin_change(vec![1, 5, 10, 25], 36), 3);
    }

    #[test]
    fn impossible() {
        assert_eq!(coin_change(vec![2], 3), -1);
    }
}
