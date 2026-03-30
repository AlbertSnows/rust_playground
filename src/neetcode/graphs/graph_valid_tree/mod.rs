pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        assert_eq!(valid_tree(5, vec![vec![0,1],vec![0,2],vec![0,3],vec![1,4]]), true);
    }

    #[test]
    fn invalid_cycle() {
        assert_eq!(valid_tree(5, vec![vec![0,1],vec![1,2],vec![2,3],vec![1,3],vec![1,4]]), false);
    }
}
