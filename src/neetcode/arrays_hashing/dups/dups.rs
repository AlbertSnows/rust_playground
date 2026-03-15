impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // nums = [1, 1, 2, 3]
        // nums = [3, 4, 5]
        // hashset = {1, 2, 3}
    }
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
