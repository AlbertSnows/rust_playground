pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    }

    #[test]
    fn another_case() {
        assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }
}
