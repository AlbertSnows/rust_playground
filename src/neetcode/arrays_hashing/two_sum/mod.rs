// use crate::common::collections::collections::some;
#![allow(dead_code)]
pub mod mod_multiple_solution_sorted;
mod mod_v1;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// Input: nums = [2,2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

fn indices_that_sum_to(
    target: i32,
    number_to_its_index: &HashMap<i32, usize>,
) -> impl Fn((usize, &i32)) -> Option<HashSet<usize>> {
    move |(subtrahend_index, subtrahend)| {
        let complement_index = number_to_its_index.get(&(target - subtrahend));
        match complement_index {
            Some(&complement_index) if complement_index != subtrahend_index => {
                Some(HashSet::from([subtrahend_index, complement_index]))
            }
            Some(&complement_index) if complement_index == subtrahend_index => None,
            _ => None,
        }
    }
}

pub fn two_sum(nums: &[i32], target: i32) -> HashSet<usize> {
    let number_to_its_index: HashMap<i32, usize> = nums
        .iter()
        .enumerate()
        .map(|(index, &number)| (number, index))
        .collect();
    let result = nums
        .iter()
        .enumerate()
        .find_map(indices_that_sum_to(target, &number_to_its_index));
    match result {
        Some(solution) => solution,
        None => unreachable!("Input should be guaranteed to have exactly one solution"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(two_sum(&vec![2, 7, 11, 15], 9), HashSet::from([0, 1]));
    }

    #[test]
    fn duplicate_values() {
        assert_eq!(two_sum(&vec![3, 3], 6), HashSet::from([1, 0]));
    }
}
