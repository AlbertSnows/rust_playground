pub fn is_happy(n: i32) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy() {
        assert_eq!(is_happy(19), true);
    }

    #[test]
    fn not_happy() {
        assert_eq!(is_happy(2), false);
    }
}
