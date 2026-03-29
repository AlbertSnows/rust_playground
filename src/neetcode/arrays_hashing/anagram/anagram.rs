// ● let s: Vec<char> = "racecar".chars().collect();
//   let mut is_palindrome = true;
//   let mut left = 0;
//   let mut right = s.len() - 1;

//   while left < right {
//       if s[left] != s[right] {
//           is_palindrome = false;
//           break;
//       }
//       left += 1;
//       right -= 1;
//   }

use crate::common::maps::maps::freq_map;

// Vec<char> -> dynamically allocated
// vs
// [char] -> slice, statically allocated
//
// [char:32] -> array, statically allocated, size known at compile time
// is_anagram(s: [char:32], t: [char:32])
// is_anagram("1111111111111111111111111111111111111111111111111111111111111111111111111111", "1111111111111111111111111111111111111111111111111111111111111111111111111111")
// not safe unless you know the size at compile time FOR ARRAYS ONLY
// only true for array, not slice
// tf is a slice?
// array size is defined at compile time.
// slice size is not known at compile time.
// but it is known at runtime.
// Slice<char>

pub fn is_anagram(s: String, t: String) -> bool {
    // aabaa
    // racecar
    let are_unequal = s.len() != t.len();
    let are_empty = s.is_empty() || t.is_empty();
    let is_one = s.len() == 1 && t.len() == 1;
    let same_letter = s == t;
    let invalid_cases = are_unequal || are_empty || (is_one && !same_letter);
    if invalid_cases {
        return false;
    }

    let s_freq = freq_map(s.chars());
    let t_freq = freq_map(t.chars());
    s_freq == t_freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_anagram() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
    }

    #[test]
    fn not_anagram() {
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }

    #[test]
    fn different_lengths() {
        assert!(!is_anagram("ab".to_string(), "a".to_string()));
    }

    #[test]
    fn single_char() {
        assert!(is_anagram("a".to_string(), "a".to_string()));
    }

    #[test]
    fn duplicate_chars() {
        assert!(is_anagram("aab".to_string(), "baa".to_string()));
    }
}
