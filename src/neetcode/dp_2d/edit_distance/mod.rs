pub fn min_distance(word1: String, word2: String) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
    }

    #[test]
    fn another() {
        assert_eq!(min_distance("intention".to_string(), "execution".to_string()), 5);
    }
}
