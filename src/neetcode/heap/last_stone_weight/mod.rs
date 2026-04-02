pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn single_stone() {
        assert_eq!(last_stone_weight(vec![1]), 1);
    }
}
