pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_conflict() {
        assert_eq!(can_attend_meetings(vec![vec![7,10],vec![2,4]]), true);
    }

    #[test]
    fn conflict() {
        assert_eq!(can_attend_meetings(vec![vec![0,30],vec![5,10],vec![15,20]]), false);
    }
}
