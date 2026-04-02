pub fn my_pow(x: f64, n: i32) -> f64 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert!((my_pow(2.0, 10) - 1024.0).abs() < 1e-9);
    }

    #[test]
    fn negative_exp() {
        assert!((my_pow(2.0, -2) - 0.25).abs() < 1e-9);
    }
}
