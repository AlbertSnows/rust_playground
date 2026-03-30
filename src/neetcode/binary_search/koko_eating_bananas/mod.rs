pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }

    #[test]
    fn another_case() {
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    }
}
