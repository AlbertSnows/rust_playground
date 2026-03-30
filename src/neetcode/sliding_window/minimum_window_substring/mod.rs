pub fn min_window(s: String, t: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC".to_string());
    }

    #[test]
    fn no_window() {
        assert_eq!(min_window("a".to_string(), "aa".to_string()), "".to_string());
    }
}
