use std::collections::HashMap;

#[derive(Default)]
pub struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        Default::default()
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut tm = TimeMap::new();
        tm.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(tm.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(tm.get("foo".to_string(), 3), "bar".to_string());
        tm.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(tm.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(tm.get("foo".to_string(), 5), "bar2".to_string());
    }

    #[test]
    fn not_found() {
        let mut tm = TimeMap::new();
        tm.set("foo".to_string(), "bar".to_string(), 2);
        assert_eq!(tm.get("foo".to_string(), 1), "".to_string());
    }
}
