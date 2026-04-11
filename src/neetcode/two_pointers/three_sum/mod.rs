// # 3Sum

// Given an integer array, return all triplets that sum to zero with no duplicate triplets.

// ## Example
// Input: nums = [-1,0,1,2,-1,-4]
// Output: [[-1,-1,2],[-1,0,1]]

use crate::common::maps::{freq_map, unroll_freq_map};
use crate::neetcode::arrays_hashing::two_sum::mod_multiple_solution_sorted::two_sum;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn get_three_sum_for_num<'a>(
    num_freq: &'a HashMap<&'a i32, usize>,
) -> impl Fn(i32) -> Vec<HashMap<i32, usize>> + 'a {
    move |num| {
        let mut two_sum_freq = num_freq.clone();
        *two_sum_freq.get_mut(&num).unwrap() -= 1;
        let two_sum_list = unroll_freq_map(&two_sum_freq);
        // sorted_two_sum_list carries original indices so two_sum can return them,
        // letting us look up values back in two_sum_list
        let sorted_two_sum_list = two_sum_list
            .iter()
            .enumerate()
            .sorted_by_key(|(_, num)| *num)
            .map(|(index, &num)| (index, num))
            .collect::<Vec<_>>();
        let two_sum_outcome = two_sum(&sorted_two_sum_list, -num);
        two_sum_outcome
            .into_iter()
            .filter_map(|two_sum_solution: HashSet<usize>| {
                // indices are original positions in two_sum_list (pre-sort)
                let pair_values: Vec<i32> = two_sum_solution
                    .iter()
                    .map(|&original_index| two_sum_list[original_index])
                    .collect();
                // only keep triplet when num is the minimum value, so each triplet is generated once
                if pair_values.iter().all(|&val| num <= val) {
                    let mut triplet_freq =
                        pair_values
                            .into_iter()
                            .fold(HashMap::new(), |mut map, val| {
                                *map.entry(val).or_insert(0) += 1;
                                map
                            });
                    *triplet_freq.entry(num).or_insert(0) += 1;
                    Some(triplet_freq)
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn three_sum(nums: Vec<i32>) -> Vec<HashMap<i32, usize>> {
    let num_freq = freq_map(nums.iter());
    let unique_nums: HashSet<i32> = nums.iter().copied().collect();
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
        let result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert!(result.contains(&HashMap::from([(-1, 2), (2, 1)])));
        assert!(result.contains(&HashMap::from([(-1, 1), (0, 1), (1, 1)])));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn no_solution() {
        assert_eq!(three_sum(vec![0, 1, 1]), vec![]);
    }
}
