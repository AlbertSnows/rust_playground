pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible() {
        assert_eq!(can_finish(2, vec![vec![1, 0]]), true);
    }

    #[test]
    fn impossible() {
        assert_eq!(can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    }
}
