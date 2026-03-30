pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let heights = vec![
            vec![1,2,2,3,5],
            vec![3,2,3,4,4],
            vec![2,4,5,3,1],
            vec![6,7,1,4,5],
            vec![5,1,1,2,4],
        ];
        let mut result = pacific_atlantic(heights);
        result.sort();
        let mut expected = vec![
            vec![0,4],vec![1,3],vec![1,4],vec![2,2],vec![3,0],vec![3,1],vec![4,0],
        ];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn single_cell() {
        assert_eq!(pacific_atlantic(vec![vec![1]]), vec![vec![0, 0]]);
    }
}
