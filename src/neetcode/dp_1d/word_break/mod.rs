pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(word_break("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]), true);
    }

    #[test]
    fn impossible() {
        assert_eq!(word_break("catsandog".to_string(), vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]), false);
    }
}
