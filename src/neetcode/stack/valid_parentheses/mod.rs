pub fn is_valid(s: String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn invalid() {
        assert_eq!(is_valid("(]".to_string()), false);
    }
}
