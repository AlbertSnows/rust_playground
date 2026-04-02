pub fn encode(strs: Vec<String>) -> String {
    Default::default()
}

pub fn decode(s: String) -> Vec<String> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = vec!["neet".to_string(), "code".to_string(), "love".to_string(), "you".to_string()];
        let encoded = encode(input.clone());
        assert_eq!(decode(encoded), input);
    }

    #[test]
    fn with_special_chars() {
        let input = vec!["we".to_string(), "say".to_string(), ":".to_string(), "yes".to_string()];
        let encoded = encode(input.clone());
        assert_eq!(decode(encoded), input);
    }
}
