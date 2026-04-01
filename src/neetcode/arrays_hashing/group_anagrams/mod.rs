use crate::common::{
    collections::traits::AddIfNotExists,
    maps::{freq_map, hash_freq_map},
};
use std::collections::{HashMap, HashSet};
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // Input: strs = ["tan", "nat","bat"]
    // Output: [["bat"],["nat","tan"]]
    // freq map -> hash the freq map?
    // [/n /a /t]
    let char_2d_list: Vec<Vec<char>> = strs
        .iter()
        .map(|initial_word| initial_word.chars().collect())
        .collect();
    // [{n: 1, a: 1, t: 1}]
    let freq_list: Vec<HashMap<&char, usize>> = char_2d_list
        .iter()
        .map(|char_list: &Vec<char>| freq_map(char_list.iter()))
        .collect();
    // ["n1a1t1"]
    let freq_hashes: Vec<String> = freq_list
        .iter()
        .map(|freq: &HashMap<&char, usize>| hash_freq_map(freq))
        .collect();
    let add_index_to_freq_hash = |mut map: HashMap<String, HashSet<usize>>,
                                  (i, hash): (usize, &String)| {
        map.entry(hash.clone())
            .or_insert_with(HashSet::new)
            .add_if_not_exists(i);
        map
    };
    let freq_hash_to_index: HashMap<String, HashSet<usize>> = freq_hashes
        .iter()
        .enumerate()
        .fold(std::collections::HashMap::new(), add_index_to_freq_hash);
    let result: Vec<Vec<String>> = freq_hash_to_index
        .values()
        .map(|indices| indices.iter().map(|&i| strs[i].clone()).collect())
        .collect();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let mut result = group_anagrams(input);
        for group in result.iter_mut() {
            group.sort();
        }
        result.sort();
        assert_eq!(
            result,
            vec![vec!["ate", "eat", "tea"], vec!["bat"], vec!["nat", "tan"],]
        );
    }

    #[test]
    fn single_word() {
        let result = group_anagrams(vec!["a".to_string()]);
        assert_eq!(result, vec![vec!["a".to_string()]]);
    }
}
