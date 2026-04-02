pub fn unique_paths(m: i32, n: i32) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(unique_paths(3, 7), 28);
    }

    #[test]
    fn square() {
        assert_eq!(unique_paths(3, 2), 3);
    }
}
