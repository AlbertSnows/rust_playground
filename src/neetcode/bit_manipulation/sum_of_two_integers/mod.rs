pub fn get_sum(a: i32, b: i32) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(get_sum(1, 2), 3);
    }

    #[test]
    fn negative() {
        assert_eq!(get_sum(2, -1), 1);
    }
}
