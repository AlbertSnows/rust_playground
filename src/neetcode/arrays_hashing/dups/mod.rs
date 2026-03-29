#[allow(dead_code)]
use crate::common::collections::traits::AddIfNotExists;
// // has_dupe = some(nums, not is_unique(hashset))
// let is_dup = |num: &i32| add_if_not_exists(&mut hashset, *num);
// // drop(is_dup); if you drop, you have to redefine the clojure
// // hashset.insert(99); illegal, bc is_dup borrows hashset
// nums.iter().any(is_dup)
// nums = [1, 1, 2, 3]
// nums = [3, 4, 5]
// hashset = {1, 2, 3}
// this doesn't work because some doesn't allow mutation in predicates
// let is_dup = |num: &i32| unique_nums.add_if_not_exists(*num);
// let has_dup = some(is_dup, &nums).is_some();
// has_dup

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut unique_nums = std::collections::HashSet::new();
    let not_unique = |num: &i32| !unique_nums.add_if_not_exists(*num);
    let has_dup = nums.iter().any(not_unique);
    has_dup
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        dbg!(contains_duplicate(vec![1, 2, 3]));
        assert_eq!(contains_duplicate(vec![1, 2, 3]), false);
    }

    #[test]
    fn duplicate_values() {
        assert_eq!(contains_duplicate(vec![3, 3]), true);
    }
}
