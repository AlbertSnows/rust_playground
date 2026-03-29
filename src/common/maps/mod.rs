use std::collections::HashMap;
use std::hash::Hash;

pub fn freq_map<T: Eq + Hash>(iter: impl Iterator<Item = T>) -> HashMap<T, usize> {
    let mut resulting_map = HashMap::new();
    for item in iter {
        *resulting_map.entry(item).or_insert(0) += 1;
    }
    resulting_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let result = freq_map(std::iter::empty::<char>());
        assert!(result.is_empty());
    }

    #[test]
    fn single_element() {
        let result = freq_map("a".chars());
        assert_eq!(result[&'a'], 1);
    }

    #[test]
    fn all_same() {
        let result = freq_map("aaaa".chars());
        assert_eq!(result[&'a'], 4);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn all_unique() {
        let result = freq_map("abcd".chars());
        assert!(result.values().all(|&v| v == 1));
    }

    #[test]
    fn mixed_frequencies() {
        let result = freq_map("aabbc".chars());
        assert_eq!(result[&'a'], 2);
        assert_eq!(result[&'b'], 2);
        assert_eq!(result[&'c'], 1);
    }

    #[test]
    fn integers() {
        let result = freq_map(vec![1, 2, 2, 3, 3, 3].into_iter());
        assert_eq!(result[&1], 1);
        assert_eq!(result[&2], 2);
        assert_eq!(result[&3], 3);
    }

    #[test]
    fn strings() {
        let result = freq_map(vec!["foo", "bar", "foo"].into_iter());
        assert_eq!(result[&"foo"], 2);
        assert_eq!(result[&"bar"], 1);
    }
}
