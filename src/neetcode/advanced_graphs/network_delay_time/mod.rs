pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(network_delay_time(vec![vec![2,1,1],vec![2,3,1],vec![3,4,1]], 4, 2), 2);
    }

    #[test]
    fn unreachable() {
        assert_eq!(network_delay_time(vec![vec![1,2,1]], 2, 2), -1);
    }
}
