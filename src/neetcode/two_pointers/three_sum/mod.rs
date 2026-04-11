// # 3Sum

// Given an integer array, return all triplets that sum to zero with no duplicate triplets.

// ## Example
// Input: nums = [-1,0,1,2,-1,-4]
// Output: [[-1,-1,2],[-1,0,1]]

use crate::{
    common::maps::{freq_map, unroll_freq_map},
    neetcode::arrays_hashing::two_sum::mod_multiple_solution_sorted::two_sum,
};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

fn get_three_sum_for_num(
    num_freq: &HashMap<&i32, usize>,
) -> impl Fn(i32) -> Vec<HashMap<i32, usize>> {
    move |num| {
        let mut two_sum_freq = num_freq.clone();
        *two_sum_freq.get_mut(&num).unwrap() -= 1;
        let two_sum_list = unroll_freq_map(&two_sum_freq);
        let sorted_two_sum_list = two_sum_list
            .iter()
            .enumerate()
            .sorted_by_key(|(_, num): &(usize, &i32)| *num)
            .map(|(index, &num)| (index, num))
            .collect::<Vec<_>>();
        let two_sum_outcome = two_sum(&sorted_two_sum_list, num);
        two_sum_outcome
            .into_iter()
            .map(|two_sum_solution: HashSet<usize>| {
                let mut triplet_freq: HashMap<i32, usize> = two_sum_solution
                    .into_iter()
                    .map(|index| sorted_two_sum_list[index].1)
                    .fold(HashMap::new(), |mut map, val| {
                        *map.entry(val).or_insert(0) += 1;
                        map
                    });
                *triplet_freq.entry(num).or_insert(0) += 1;
                triplet_freq
            })
            .collect()
    }
}

// -1, 0, 1, 2, -1, 2, -4
// -4 -1 -1 0 1 2 2
// -4 1, -1 2, 0 1, 1 1, 2 2
pub fn three_sum(nums: Vec<i32>) -> Vec<HashMap<i32, usize>> {
    let unique_nums = nums.into_iter().collect::<std::collections::HashSet<_>>();
    let num_freq = freq_map(unique_nums.iter());
    unique_nums
        .iter()
        .copied()
        .flat_map(get_three_sum_for_num(&num_freq))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        result.sort();
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn no_solution() {
        assert_eq!(three_sum(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>);
    }
}
