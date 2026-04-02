use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Option<Rc<RefCell<Node>>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node { val, neighbors: vec![] }
    }
}

pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(clone_graph(None).is_none());
    }

    #[test]
    fn single_node() {
        let n = Some(Rc::new(RefCell::new(Node::new(1))));
        let result = clone_graph(n.clone());
        assert!(result.is_some());
        assert_eq!(result.unwrap().borrow().val, 1);
    }
}
