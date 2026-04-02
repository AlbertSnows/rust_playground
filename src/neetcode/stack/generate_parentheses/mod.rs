pub fn generate_parenthesis(n: i32) -> Vec<String> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_equals_1() {
        assert_eq!(generate_parenthesis(1), vec!["()".to_string()]);
    }

    #[test]
    fn n_equals_3() {
        let mut result = generate_parenthesis(3);
        result.sort();
        let mut expected = vec!["((()))","(()())","(())()","()(())","()()()"];
        expected.sort();
        assert_eq!(result, expected);
    }
}
