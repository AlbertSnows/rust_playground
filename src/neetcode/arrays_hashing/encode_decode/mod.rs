// Design an algorithm to encode a list of strings to a single string and decode it back to the original list.

// ## Example
// Input: ["neet","code","love","you"]
// Encoded: "4#neet4#code4#love3#you"
// Output: ["neet","code","love","you"]

pub fn encode(strs: Vec<String>) -> String {
    // [(neet, 4), (code, 4), (love, 4), (you, 3)]
    // 4#neet, ...
    let outcome = strs
        .iter()
        .map(|s| format!("{}#{}", s.len(), s))
        .collect::<Vec<String>>()
        .join(",");
    outcome
}

pub fn decode(s: String) -> Vec<String> {
    // 4#neet
    // [neet, code, ]
    let str_len = s.len();
    let mut current_number_index = 0;
    let mut result = vec![];
    while current_number_index < str_len {
        // 4#neet10#oneoneone
        let j = s[current_number_index..].find('#').unwrap() + current_number_index;
        let len: usize = s[current_number_index..j].parse().unwrap();
        result.push(s[j + 1..j + 1 + len].to_string());
        current_number_index = j + 1 + len;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = vec![
            "neet".to_string(),
            "code".to_string(),
            "love".to_string(),
            "you".to_string(),
        ];
        let encoded = encode(input.clone());
        assert_eq!(decode(encoded), input);
    }

    #[test]
    fn with_special_chars() {
        let input = vec![
            "we".to_string(),
            "say".to_string(),
            ":".to_string(),
            "yes".to_string(),
        ];
        let encoded = encode(input.clone());
        assert_eq!(decode(encoded), input);
    }
}
