pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    }

    #[test]
    fn four_courses() {
        let result = find_order(4, vec![vec![1,0],vec![2,0],vec![3,1],vec![3,2]]);
        assert_eq!(result.len(), 4);
    }
}
