// # Longest Consecutive Sequence

// Given an unsorted array of integers, return the length of the longest consecutive elements sequence in O(n) time.

// ## Example
// Input: nums = [100,4,200,1,3,2]
// Output: 4 (the sequence [1,2,3,4])

use std::collections::HashMap;
use std::collections::HashSet;

pub fn traverse(
    mut incriment_length_for_num: HashMap<i32, usize>,
    nums: HashSet<i32>,
    num_to_check: i32,
    current_length: usize,
) -> HashMap<i32, usize> {
    let can_continue = nums.contains(&num_to_check);
    if !can_continue {
        return incriment_length_for_num;
    }
    incriment_length_for_num
        .entry(num_to_check)
        .and_modify(|v| *v += 1)
        .or_insert(1);
    traverse(
        incriment_length_for_num,
        nums,
        num_to_check + 1,
        current_length + 1,
    )
}

pub fn longest_consecutive(nums: Vec<i32>) -> usize {
    let unique_nums = nums.into_iter().collect::<std::collections::HashSet<i32>>();
    let incriment_length_for_num = unique_nums
        .clone()
        .into_iter()
        .fold(Default::default(), |acc, num| {
            traverse(acc, unique_nums.clone(), num, 0)
        });
    incriment_length_for_num
        .values()
        .max()
        .copied()
        .unwrap_or(0) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn longer_sequence() {
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}
