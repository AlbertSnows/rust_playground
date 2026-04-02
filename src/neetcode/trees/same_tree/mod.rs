use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same() {
        let p = node_with_children(1, node(2), node(3));
        let q = node_with_children(1, node(2), node(3));
        assert_eq!(is_same_tree(p, q), true);
    }

    #[test]
    fn different() {
        let p = node_with_children(1, node(2), None);
        let q = node_with_children(1, None, node(2));
        assert_eq!(is_same_tree(p, q), false);
    }
}
