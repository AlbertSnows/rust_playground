pub fn check_valid_string(s: String) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(check_valid_string("(*))".to_string()), true);
    }

    #[test]
    fn invalid() {
        assert_eq!(check_valid_string(")".to_string()), false);
    }
}
