// use crate::common::collections::collections::some;
#![allow(dead_code)]
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// Given an array of integers nums and an integer target,
// return indices of the two numbers such that they add up to target.
//
// Input: nums = [3,2,4], target = 6
// Output: [1,2]

// match ergonomics
// &(index, &num): &(usize, &i32)  // explicit
// (index, &num): &(usize, &i32)   // implicit — Rust auto-derefs the outer &
//
//
// So fold's closure signature is:
// ```rust
// |accumulator, item| -> accumulator
// ```
// ```
// fold gives you the map → you modify it → you hand it back → fold gives it to next iteration
// ```
// vs `&mut` which is more like:
// ```
// you own the map → you let someone borrow it temporarily → they give it back to you
// let mut num_to_original_index: HashMap<i32, HashSet<usize>> = nums
//     .iter() // &i32
//     .enumerate() // (usize, &i32)
//     .fold(HashMap::new(), add_to_multimap); // passes &(usize, &i32)

// let mut sorted_nums = nums.clone();
// sorted_nums.sort();
// let mut left = 0;
// let mut right = sorted_nums.len() - 1;
// let still_searching = |left, right, target| {
//     left < right
//         && sorted_nums[left] + sorted_nums[right] != target
//         && sorted_nums[left] <= target
// };
// while still_searching(left, right, target) {
//     if sorted_nums[left] + sorted_nums[right] < target {
//         left += 1;
//     } else if sorted_nums[left] + sorted_nums[right] > target {
//         right -= 1;
//     } else {
//         break;
//     }
// }
// // t = 2
// // o = [1, 3, 0, 1]
// // v = [0, 1, 1, 3]
// // m = {
// // 1: [0, 3],
// // 3: [1],
// // 0: [2]
// // }
// // v[1], v[2] => 1, 3 => m[v[1]][?], ??] =>
// let left_index = num_to_original_index[&sorted_nums[left]];
// let right_index = num_to_original_index[&sorted_nums[right]];
// return vec![left_index as i32, right_index as i32];

// struct TwoSumInfo {
//     indexes: HashSet<usize>,
//     remainder: i32,
//     partner: HashSet<i32>,
// }

// impl TwoSumInfo {
//     fn new(index: usize, remainder: i32) -> Self {
//         Self {
//             indexes: HashSet::from([index]),
//             remainder,
//             partner: HashSet::new(),
//         }
//     }

//     fn add_info(&mut self, index: usize, num: i32) {
//         self.indexes.insert(index);
//         self.remainder -= num;
//         self.partner.insert(num);
//     }
// }

// (defn define_two_sum_info [index, remainder]
//     {
//         indexes: {index},
//     remainder: remainder,
//     partner: nil
//     })

// let two_sum_info = define_two_sum_info(1, 3);
// {
//     indexes: {1},
//     remainder: 3,
//     partner: nil,
// }

// (upsert :partner {4} two_sum_info)
// one or more solutions, return all?
// [4, 3, 5, 3, 4, 1, 3]
// target = 6
// [
// 4: {index: {0, 4}, remainder: 2, partner: null},
// 3: {index: {1, 3, 6}, remainder: 3, partner: {1,3,6}},
// 5: {index: {2}, remainder: 1, partner: 1}
// 1: {index: {5}, remainder: 5, partner: 5}
// ]
// 6 - # = key

// 4, 3, 5, 1

pub fn map_value_to_indexes(nums: &[i32]) -> HashMap<i32, Vec<usize>> {
    nums.iter()
        .enumerate() // [(0, 4), (1, 3), ...]
        .map(|(idx, &num)| (num, idx))
        .into_group_map()
}

pub fn two_sum(nums: &[i32], target: i32) -> HashSet<usize> {
    let num_to_indexes = map_value_to_indexes(nums);
    let has_complement = |&num: &i32| num_to_indexes.contains_key(&(target - num));
    // num_to_indexes.keys() = Iterator<Item = &i32>
    // some returns Option<&i32>
    let valid_num = (num_to_indexes.keys().copied()).find(has_complement); //some(has_complement, num_to_indexes.keys()); // .copied());
    let get_valid_pair = |num| -> Vec<usize> {
        let complement = target - num;
        let is_self_compliment = num == complement;
        let indexes = num_to_indexes.get(&num).unwrap();
        let complement_indexes = num_to_indexes.get(&complement).unwrap();
        let valid_pair = if is_self_compliment {
            vec![indexes[0], indexes[1]]
        } else {
            vec![indexes[0], complement_indexes[0]]
        };
        valid_pair
    };

    let valid_pair = get_valid_pair(valid_num.unwrap());
    valid_pair.into_iter().collect::<HashSet<usize>>()
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
