pub fn count_substrings(s: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(count_substrings("abc".to_string()), 3);
    }

    #[test]
    fn with_palindromes() {
        assert_eq!(count_substrings("aaa".to_string()), 6);
    }
}
