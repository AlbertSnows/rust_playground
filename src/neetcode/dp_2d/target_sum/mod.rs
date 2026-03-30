pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }

    #[test]
    fn single_element() {
        assert_eq!(find_target_sum_ways(vec![1], 1), 1);
    }
}
