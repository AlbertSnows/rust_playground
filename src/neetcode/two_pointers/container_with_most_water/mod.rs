pub fn max_area(height: Vec<i32>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn two_elements() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}
