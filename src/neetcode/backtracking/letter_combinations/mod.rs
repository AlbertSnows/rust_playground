pub fn letter_combinations(digits: String) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut result = letter_combinations("23".to_string());
        result.sort();
        let mut expected = vec!["ad","ae","af","bd","be","bf","cd","ce","cf"];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn empty() {
        assert_eq!(letter_combinations("".to_string()), vec![] as Vec<String>);
    }
}
