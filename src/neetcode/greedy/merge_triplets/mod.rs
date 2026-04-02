pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible() {
        assert_eq!(merge_triplets(vec![vec![2,5,3],vec![1,8,4],vec![1,7,5]], vec![2,7,5]), true);
    }

    #[test]
    fn impossible() {
        assert_eq!(merge_triplets(vec![vec![3,4,5],vec![4,5,6]], vec![3,2,5]), false);
    }
}
