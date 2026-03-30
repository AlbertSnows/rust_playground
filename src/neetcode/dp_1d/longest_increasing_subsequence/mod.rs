pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn all_same() {
        assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }
}
