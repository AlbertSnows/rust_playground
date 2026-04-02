pub fn check_inclusion(s1: String, s2: String) -> bool {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(check_inclusion("ab".to_string(), "eidbaooo".to_string()), true);
    }

    #[test]
    fn no_inclusion() {
        assert_eq!(check_inclusion("ab".to_string(), "eidboaoo".to_string()), false);
    }
}
