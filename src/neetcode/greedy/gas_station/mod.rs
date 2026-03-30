pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(can_complete_circuit(vec![1,2,3,4,5], vec![3,4,5,1,2]), 3);
    }

    #[test]
    fn impossible() {
        assert_eq!(can_complete_circuit(vec![2,3,4], vec![3,4,3]), -1);
    }
}
