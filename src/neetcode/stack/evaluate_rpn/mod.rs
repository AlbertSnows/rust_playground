pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let tokens = vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()];
        assert_eq!(eval_rpn(tokens), 9);
    }

    #[test]
    fn division() {
        let tokens = vec!["4".to_string(), "13".to_string(), "5".to_string(), "/".to_string(), "+".to_string()];
        assert_eq!(eval_rpn(tokens), 6);
    }
}
