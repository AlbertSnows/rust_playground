pub fn num_decodings(s: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(num_decodings("12".to_string()), 2);
    }

    #[test]
    fn with_zeros() {
        assert_eq!(num_decodings("226".to_string()), 3);
    }
}
