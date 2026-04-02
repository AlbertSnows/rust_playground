pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut matrix = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1,0,1],vec![0,0,0],vec![1,0,1]]);
    }

    #[test]
    fn corner() {
        let mut matrix = vec![vec![0,1,2,0],vec![3,4,5,2],vec![1,3,1,5]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0,0,0,0],vec![0,4,5,0],vec![0,3,1,0]]);
    }
}
