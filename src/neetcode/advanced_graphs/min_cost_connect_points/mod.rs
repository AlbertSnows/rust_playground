pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(min_cost_connect_points(vec![vec![0,0],vec![2,2],vec![3,10],vec![5,2],vec![7,0]]), 20);
    }

    #[test]
    fn two_points() {
        assert_eq!(min_cost_connect_points(vec![vec![0,0],vec![1,1]]), 2);
    }
}
