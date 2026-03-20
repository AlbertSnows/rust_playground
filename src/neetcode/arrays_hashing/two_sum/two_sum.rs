use std::collections::{HashMap, HashSet};

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

pub fn add_to_multimap(
    mut map: HashMap<i32, HashSet<usize>>,
    (index, num): (i32, usize),
) -> HashMap<i32, HashSet<usize>> {
    map.entry(index).or_insert_with(HashSet::new).insert(num);
    map
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // [3, 3, 2, 4]
    // target = 6
    // [
    // 3:[0, 1],
    // 4:[2],
    // 2:[3]
    // ]
    let needed_to_indexes = nums
        .iter()
        .enumerate()
        .map(|(index, num)| (target - num, index))
        .fold(HashMap::new(), add_to_multimap);

    for (index, num) in nums.iter().enumerate() {
        let needed = target - num;
        let indexes = needed_to_indexes.get(&needed);
        let other_index =
            indexes.is_some() && indexes.unwrap().iter().find(|&&idx| idx != index).is_some();
        // todo: finish
        if other_index {
            return vec![
                index as i32,
                indexes.unwrap().iter().find(|&&idx| idx != index).unwrap() as i32,
            ];
        }
    }

    // for index, num in nums
    // 0, 3 | 1, 3 |
    // needed = 6 - 3 = 3
    // indexes = needed_to_indexes.get(needed)
    // and !x.contains(index)
    return vec![];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn duplicate_values() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
