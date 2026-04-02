pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(min_meeting_rooms(vec![vec![0,30],vec![5,10],vec![15,20]]), 2);
    }

    #[test]
    fn no_overlap() {
        assert_eq!(min_meeting_rooms(vec![vec![7,10],vec![2,4]]), 1);
    }
}
