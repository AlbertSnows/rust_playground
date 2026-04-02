pub fn count_bits(n: i32) -> Vec<i32> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn five() {
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
