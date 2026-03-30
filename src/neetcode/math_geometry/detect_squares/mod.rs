use std::collections::HashMap;

pub struct DetectSquares {
    points: HashMap<(i32, i32), i32>,
}

impl DetectSquares {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add(&mut self, point: Vec<i32>) {
        todo!()
    }

    pub fn count(&self, point: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut ds = DetectSquares::new();
        ds.add(vec![3, 10]);
        ds.add(vec![11, 2]);
        ds.add(vec![3, 2]);
        assert_eq!(ds.count(vec![11, 10]), 1);
        assert_eq!(ds.count(vec![14, 8]), 0);
        ds.add(vec![11, 2]);
        assert_eq!(ds.count(vec![11, 10]), 2);
    }

    #[test]
    fn no_square() {
        let ds = DetectSquares::new();
        assert_eq!(ds.count(vec![0, 0]), 0);
    }
}
