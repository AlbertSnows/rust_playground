pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(find_redundant_connection(vec![vec![1,2],vec![1,3],vec![2,3]]), vec![2,3]);
    }

    #[test]
    fn longer() {
        assert_eq!(find_redundant_connection(vec![vec![1,2],vec![2,3],vec![3,4],vec![1,4],vec![1,5]]), vec![1,4]);
    }
}
