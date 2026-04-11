#![allow(dead_code)]

// Input: nums = [2,7,11,15], target = 9
// Output: [[0,1]]
// Explanation: Because nums[0] + nums[1] == 9, we return [[0, 1]].

// fn build_index_map(nums: &[i32]) -> HashMap<i32, Vec<usize>> {
//     nums.iter()
//         .enumerate()
//         .fold(HashMap::new(), |mut map, (index, &number)| {
//             map.entry(number).or_default().push(index);
//             map
//         })
// }

// fn pairs_with_complement(
//     candidate_index: usize,
//     candidate: i32,
//     target: i32,
//     index_map: &HashMap<i32, Vec<usize>>,
// ) -> impl Iterator<Item = HashSet<usize>> + '_ {
//     // 4 5 2 4, target = 9,
//     let complement = target - candidate;
//     let not_yet_seen = |&&complement_index: &&usize| complement_index > candidate_index;
//     index_map
//         .get(&complement)
//         // into_iter().flatten() = "if this exists, iterate its contents; otherwise produce nothing"
//         .into_iter() // => [[foo, bar]]
//         // into_iter yields iter of &vec<usize>
//         // flatten converts to iter of &usize
//         .flatten() // => [foo, bar]
//         .filter(move |&&complement_index| complement_index > candidate_index)
//         .map(move |&complement_index| HashSet::from([candidate_index, complement_index]))
// }

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

struct TwoSumState {
    left: usize,
    right: usize,
    pairs: Vec<HashSet<usize>>,
}

fn iterate(state: TwoSumState, sorted_nums: &[(usize, i32)], target: i32) -> TwoSumState {
    let (left_index, left_val) = sorted_nums[state.left];
    let (right_index, right_val) = sorted_nums[state.right];
    let total_compared_to_target = (left_val + right_val).cmp(&target);
    match total_compared_to_target {
        Ordering::Less => TwoSumState {
            left: state.left + 1,
            ..state // = create a new state with left set to blah and copy the rest from state
        },
        Ordering::Greater => TwoSumState {
            right: state.right - 1,
            ..state
        },
        Ordering::Equal => TwoSumState {
            left: state.left + 1,
            right: state.right - 1,
            pairs: state
                .pairs
                .into_iter()
                .chain([HashSet::from([left_index, right_index])])
                .collect(),
        },
    }
}

pub fn two_sum(sorted_nums: &[(usize, i32)], target: i32) -> Vec<HashSet<usize>> {
    let mut state = TwoSumState {
        left: 0,
        right: sorted_nums.len() - 1,
        pairs: Vec::new(),
    };
    while state.left < state.right {
        state = iterate(state, &sorted_nums, target);
    }
    state.pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted_indexed(nums: &[i32]) -> Vec<(usize, i32)> {
        nums.iter()
            .enumerate()
            .sorted_by_key(|&(_, num)| *num)
            .map(|(index, &num)| (index, num))
            .collect()
    }

    #[test]
    fn basic() {
        // [2, 7, 11, 15] already sorted → [(0,2), (1,7), (2,11), (3,15)]
        let input = sorted_indexed(&[2, 7, 11, 15]);
        assert_eq!(two_sum(&input, 9), vec![HashSet::from([0, 1])]);
    }

    #[test]
    fn duplicate_values() {
        // [3, 3] → [(0,3), (1,3)]
        let input = sorted_indexed(&[3, 3]);
        assert_eq!(two_sum(&input, 6), vec![HashSet::from([0, 1])]);
    }

    #[test]
    fn multiple_solutions() {
        // [1, 3, 2, 4] sorted → [(0,1), (2,2), (1,3), (3,4)]
        let input = sorted_indexed(&[1, 3, 2, 4]);
        let mut result = two_sum(&input, 5);
        result.sort_by_key(|set| *set.iter().min().unwrap());
        assert_eq!(result, vec![HashSet::from([0, 3]), HashSet::from([1, 2])]);
    }

    #[test]
    fn duplicate_values_multiple_solutions() {
        // [1, 1, 2, 2] sorted → [(0,1), (1,1), (2,2), (3,2)]
        // two-pointer finds non-overlapping pairs only: {0,3} and {1,2}
        let input = sorted_indexed(&[1, 1, 2, 2]);
        let mut result = two_sum(&input, 3);
        result.sort_by_key(|set| *set.iter().min().unwrap());
        assert_eq!(
            result,
            vec![HashSet::from([0, 3]), HashSet::from([1, 2])]
        );
    }
}
