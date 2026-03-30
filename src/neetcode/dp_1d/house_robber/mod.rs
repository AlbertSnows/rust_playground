pub fn rob(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn another() {
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
