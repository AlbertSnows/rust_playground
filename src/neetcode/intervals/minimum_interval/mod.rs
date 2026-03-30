pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            min_interval(vec![vec![1,4],vec![2,4],vec![3,6],vec![4,4]], vec![2,3,4,5]),
            vec![3,3,1,4]
        );
    }

    #[test]
    fn no_match() {
        assert_eq!(
            min_interval(vec![vec![2,3],vec![2,5],vec![1,8],vec![20,25]], vec![2,19,22]),
            vec![2,8,6]
        );
    }
}
