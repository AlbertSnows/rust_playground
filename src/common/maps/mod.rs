use itertools::Itertools;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::repeat;
use std::vec::IntoIter;

pub fn has_dupes<T: Eq + Hash>(iter: impl Iterator<Item = T>) -> bool {
    iter.duplicates().next().is_some()
}

pub fn hash_freq_map<T: Eq + Hash + ToString + Clone + Ord>(map: &HashMap<T, usize>) -> String {
    //Keys<'_, K, V> -> '_ = lifetime
    // '_ means "this iterator cannot outlive the map it came from
    // - Iter<'_, T> — borrows, yields &T
    // - IterMut<'_, T> — borrows mutably, yields &mut T
    // - IntoIter<T> — owns, yields T
    let sorted_keys = map.keys().cloned().sorted();
    let group_key_to_freq = |k: T| k.to_string() + &map.get(&k).copied().unwrap_or(0).to_string();
    let hashed_freq_map = sorted_keys
        .into_iter()
        .map(group_key_to_freq)
        .collect::<Vec<_>>()
        .join("");
    hashed_freq_map
}

pub fn freq_map<T: Eq + Hash>(iter: impl Iterator<Item = T>) -> HashMap<T, usize> {
    let mut resulting_map = HashMap::new();
    for item in iter {
        *resulting_map.entry(item).or_insert(0) += 1;
    }
    resulting_map
}

pub fn unroll_freq_map<T: Eq + Hash + ToString + Clone + Ord>(
    freq_mapping: &HashMap<&T, usize>,
) -> Vec<T> {
    let result = freq_mapping
        .iter()
        .flat_map(|(&elem, &freq)| repeat(elem.clone()).take(freq))
        .collect::<Vec<_>>();
    result
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

    #[test]
    fn hash_freq_map_empty() {
        let map = freq_map(std::iter::empty::<char>());
        assert_eq!(hash_freq_map(&map), "");
    }

    #[test]
    fn hash_freq_map_single() {
        let map = freq_map("a".chars());
        assert_eq!(hash_freq_map(&map), "a1");
    }

    #[test]
    fn hash_freq_map_sorted_alphabetically() {
        let map = freq_map("bac".chars());
        assert_eq!(hash_freq_map(&map), "a1b1c1");
    }

    #[test]
    fn hash_freq_map_anagram_equivalence() {
        let map1 = freq_map("anagram".chars());
        let map2 = freq_map("nagaram".chars());
        assert_eq!(hash_freq_map(&map1), hash_freq_map(&map2));
    }

    #[test]
    fn hash_freq_map_non_anagram_differs() {
        let map1 = freq_map("rat".chars());
        let map2 = freq_map("car".chars());
        assert_ne!(hash_freq_map(&map1), hash_freq_map(&map2));
    }

    #[test]
    fn hash_freq_map_multiple_frequencies() {
        let map = freq_map("aabbb".chars());
        assert_eq!(hash_freq_map(&map), "a2b3");
    }
}
