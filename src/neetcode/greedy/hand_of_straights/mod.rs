pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible() {
        assert_eq!(is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3), true);
    }

    #[test]
    fn impossible() {
        assert_eq!(is_n_straight_hand(vec![1, 2, 3, 4, 5], 4), false);
    }
}
