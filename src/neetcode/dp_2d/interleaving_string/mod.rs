pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbcbcac".to_string()), true);
    }

    #[test]
    fn not_interleave() {
        assert_eq!(is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbbaccc".to_string()), false);
    }
}
