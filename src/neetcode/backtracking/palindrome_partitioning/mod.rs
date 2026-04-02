pub fn partition(s: String) -> Vec<Vec<String>> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut result = partition("aab".to_string());
        result.sort();
        let mut expected = vec![vec!["a","a","b"], vec!["aa","b"]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn single_char() {
        assert_eq!(partition("a".to_string()), vec![vec!["a".to_string()]]);
    }
}
