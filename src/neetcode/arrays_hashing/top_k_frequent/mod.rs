pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut result = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn single_element() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}
