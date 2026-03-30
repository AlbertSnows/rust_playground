pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
        let mut result = group_anagrams(input);
        for group in result.iter_mut() { group.sort(); }
        result.sort();
        assert_eq!(result, vec![
            vec!["ate", "eat", "tea"],
            vec!["bat"],
            vec!["nat", "tan"],
        ]);
    }

    #[test]
    fn single_word() {
        let result = group_anagrams(vec!["a".to_string()]);
        assert_eq!(result, vec![vec!["a".to_string()]]);
    }
}
