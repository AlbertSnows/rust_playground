pub fn num_distinct(s: String, t: String) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
    }

    #[test]
    fn another() {
        assert_eq!(num_distinct("babgbag".to_string(), "bag".to_string()), 5);
    }
}
