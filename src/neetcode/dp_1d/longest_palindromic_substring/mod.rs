pub fn longest_palindrome(s: String) -> String {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
    }

    #[test]
    fn even_palindrome() {
        assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
    }
}
