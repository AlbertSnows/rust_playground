use std::collections::HashMap;

pub struct WordDictionary {
    children: HashMap<char, WordDictionary>,
    is_end: bool,
}

impl WordDictionary {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_word(&mut self, word: String) {
        todo!()
    }

    pub fn search(&self, word: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut wd = WordDictionary::new();
        wd.add_word("bad".to_string());
        wd.add_word("dad".to_string());
        wd.add_word("mad".to_string());
        assert_eq!(wd.search("pad".to_string()), false);
        assert_eq!(wd.search("bad".to_string()), true);
        assert_eq!(wd.search(".ad".to_string()), true);
        assert_eq!(wd.search("b..".to_string()), true);
    }

    #[test]
    fn wildcard_no_match() {
        let mut wd = WordDictionary::new();
        wd.add_word("a".to_string());
        assert_eq!(wd.search("aa".to_string()), false);
    }
}
