pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn two_bars() {
        assert_eq!(largest_rectangle_area(vec![2, 4]), 4);
    }
}
