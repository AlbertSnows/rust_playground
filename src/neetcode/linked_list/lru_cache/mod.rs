use std::collections::HashMap;

#[derive(Default)]
pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, i32>,
    order: std::collections::VecDeque<i32>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Default::default()
    }

    pub fn get(&mut self, key: i32) -> i32 {
        Default::default()
    }

    pub fn put(&mut self, key: i32, value: i32) {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn not_found() {
        let mut cache = LRUCache::new(1);
        assert_eq!(cache.get(99), -1);
    }
}
