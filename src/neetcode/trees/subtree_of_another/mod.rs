use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_subtree_test() {
        let root = node_with_children(3,
            node_with_children(4, node(1), node(2)),
            node(5),
        );
        let sub = node_with_children(4, node(1), node(2));
        assert_eq!(is_subtree(root, sub), true);
    }

    #[test]
    fn not_subtree() {
        let root = node_with_children(3,
            node_with_children(4, node(1), node_with_children(2, node(0), None)),
            node(5),
        );
        let sub = node_with_children(4, node(1), node(2));
        assert_eq!(is_subtree(root, sub), false);
    }
}
