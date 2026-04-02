pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn carry() {
        assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}
