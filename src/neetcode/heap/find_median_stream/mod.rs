use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Default)]
pub struct MedianFinder {
    lower: BinaryHeap<i32>,
    upper: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_num(&mut self, num: i32) {
        Default::default()
    }

    pub fn find_median(&self) -> f64 {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);
    }

    #[test]
    fn single() {
        let mut mf = MedianFinder::new();
        mf.add_num(5);
        assert_eq!(mf.find_median(), 5.0);
    }
}
