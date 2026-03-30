pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            ladder_length("hit".to_string(), "cog".to_string(),
                vec!["hot","dot","dog","lot","log","cog"].into_iter().map(String::from).collect()),
            5
        );
    }

    #[test]
    fn no_path() {
        assert_eq!(
            ladder_length("hit".to_string(), "cog".to_string(),
                vec!["hot","dot","dog","lot","log"].into_iter().map(String::from).collect()),
            0
        );
    }
}
