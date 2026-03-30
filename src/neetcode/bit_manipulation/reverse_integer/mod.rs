pub fn reverse_integer(x: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(reverse_integer(123), 321);
    }

    #[test]
    fn negative() {
        assert_eq!(reverse_integer(-123), -321);
    }
}
