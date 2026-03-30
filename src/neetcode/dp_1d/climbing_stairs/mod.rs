pub fn climb_stairs(n: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn three_steps() {
        assert_eq!(climb_stairs(3), 3);
    }
}
