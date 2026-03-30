pub fn is_match(s: String, p: String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn star_match() {
        assert_eq!(is_match("aa".to_string(), "a*".to_string()), true);
    }
}
