pub fn is_palindrome(s: String) -> bool {
    if s.is_empty() {
        return true;
    }
    //[(0, s.len() - 1), (1, s.len() - 2), ..., (s.len() / 2, s.len() / 2)]
    let s_as_chars = s.chars();
    let list_of_alleged_matches: Vec<(char, char)> = s_as_chars
        .clone()
        .zip(s.clone().chars().rev())
        .collect::<Vec<_>>();
    let mut indexes = (0..s_as_chars.count() - 1).into_iter();
    let letters_match = |i: usize| list_of_alleged_matches[i].0 == list_of_alleged_matches[i].1;
    let all_letters_match = indexes.all(letters_match);
    all_letters_match
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_palindrome() {
        assert!(is_palindrome("racecar".to_string()));
    }

    #[test]
    fn not_palindrome() {
        assert!(!is_palindrome("hello".to_string()));
    }

    #[test]
    fn single_char() {
        assert!(is_palindrome("a".to_string()));
    }

    #[test]
    fn empty_string() {
        assert!(is_palindrome("".to_string()));
    }
}
