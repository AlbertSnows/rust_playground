pub fn missing_number(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn end_missing() {
        assert_eq!(missing_number(vec![0, 1]), 2);
    }
}
