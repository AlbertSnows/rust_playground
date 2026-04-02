use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Default)]
pub struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        Default::default()
    }

    pub fn add(&mut self, val: i32) -> i32 {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut kl = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kl.add(3), 4);
        assert_eq!(kl.add(5), 5);
        assert_eq!(kl.add(10), 5);
        assert_eq!(kl.add(9), 8);
        assert_eq!(kl.add(4), 8);
    }

    #[test]
    fn single_element() {
        let mut kl = KthLargest::new(1, vec![]);
        assert_eq!(kl.add(5), 5);
        assert_eq!(kl.add(3), 5);
    }
}
