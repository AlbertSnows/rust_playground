pub fn partition_labels(s: String) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(partition_labels("ababcbacadefegdehijhklij".to_string()), vec![9, 7, 8]);
    }

    #[test]
    fn single_chars() {
        assert_eq!(partition_labels("eccbbbbdec".to_string()), vec![10]);
    }
}
