pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(change(5, vec![1, 2, 5]), 4);
    }

    #[test]
    fn impossible() {
        assert_eq!(change(3, vec![2]), 0);
    }
}
