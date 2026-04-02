pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let board = vec![
            vec!['o','a','a','n'],
            vec!['e','t','a','e'],
            vec!['i','h','k','r'],
            vec!['i','f','l','v'],
        ];
        let words = vec!["oath".to_string(), "pea".to_string(), "eat".to_string(), "rain".to_string()];
        let mut result = find_words(board, words);
        result.sort();
        assert_eq!(result, vec!["eat".to_string(), "oath".to_string()]);
    }

    #[test]
    fn no_match() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let words = vec!["abcb".to_string()];
        assert_eq!(find_words(board, words), vec![] as Vec<String>);
    }
}
