pub fn character_replacement(s: String, k: i32) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(character_replacement("ABAB".to_string(), 2), 4);
    }

    #[test]
    fn another_case() {
        assert_eq!(character_replacement("AABABBA".to_string(), 1), 4);
    }
}
