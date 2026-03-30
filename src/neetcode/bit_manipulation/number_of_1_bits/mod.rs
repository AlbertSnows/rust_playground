pub fn hamming_weight(n: u32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(hamming_weight(0b00000000000000000000000000001011), 3);
    }

    #[test]
    fn all_ones() {
        assert_eq!(hamming_weight(0b11111111111111111111111111111101), 31);
    }
}
