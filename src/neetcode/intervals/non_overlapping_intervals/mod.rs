pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(erase_overlap_intervals(vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]]), 1);
    }

    #[test]
    fn no_overlap() {
        assert_eq!(erase_overlap_intervals(vec![vec![1,2],vec![2,3]]), 0);
    }
}
