pub fn two_sum_ii(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(two_sum_ii(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn adjacent() {
        assert_eq!(two_sum_ii(vec![2, 3, 4], 6), vec![1, 3]);
    }
}
