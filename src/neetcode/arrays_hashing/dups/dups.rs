#[allow(dead_code)]
pub fn add_if_not_exists(hashset: &mut std::collections::HashSet<i32>, num: i32) -> bool {
    if hashset.contains(&num) {
        return true;
    }
    hashset.insert(num);
    false
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hashset = std::collections::HashSet::new();
    let is_dup = |num: &i32| add_if_not_exists(&mut hashset, *num);
    // drop(is_dup); if you drop, you have to redefine the clojure
    // hashset.insert(99); illegal, bc is_dup borrows hashset
    nums.iter().any(is_dup)
    // nums = [1, 1, 2, 3]
    // nums = [3, 4, 5]
    // hashset = {1, 2, 3}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(contains_duplicate(vec![1, 2, 3]), false);
    }

    #[test]
    fn duplicate_values() {
        assert_eq!(contains_duplicate(vec![3, 3]), true);
    }
}
