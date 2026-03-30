pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            insert(vec![vec![1,3],vec![6,9]], vec![2,5]),
            vec![vec![1,5],vec![6,9]]
        );
    }

    #[test]
    fn merge_all() {
        assert_eq!(
            insert(vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]], vec![4,8]),
            vec![vec![1,2],vec![3,10],vec![12,16]]
        );
    }
}
