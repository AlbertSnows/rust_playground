pub fn multiply(num1: String, num2: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(multiply("2".to_string(), "3".to_string()), "6".to_string());
    }

    #[test]
    fn larger() {
        assert_eq!(multiply("123".to_string(), "456".to_string()), "56088".to_string());
    }
}
