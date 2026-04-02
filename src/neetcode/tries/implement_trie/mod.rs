use std::collections::HashMap;

#[derive(Default)]
pub struct Trie {
    children: HashMap<char, Trie>,
    is_end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, word: String) {
        Default::default()
    }

    pub fn search(&self, word: String) -> bool {
        Default::default()
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple".to_string()), true);
        assert_eq!(trie.search("app".to_string()), false);
        assert_eq!(trie.starts_with("app".to_string()), true);
    }

    #[test]
    fn insert_and_find() {
        let mut trie = Trie::new();
        trie.insert("app".to_string());
        assert_eq!(trie.search("app".to_string()), true);
        assert_eq!(trie.starts_with("ap".to_string()), true);
    }
}
