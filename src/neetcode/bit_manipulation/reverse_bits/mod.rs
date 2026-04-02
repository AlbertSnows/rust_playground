pub fn reverse_bits(x: u32) -> u32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(reverse_bits(0b00000010100101000001111010011100), 0b00111001011110000010100101000000);
    }

    #[test]
    fn all_ones() {
        assert_eq!(reverse_bits(0xFFFFFFFF), 0xFFFFFFFF);
    }
}
