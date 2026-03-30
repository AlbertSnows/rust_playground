use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let tree = node_with_children(3, node(1), node(4));
        assert_eq!(kth_smallest(tree, 1), 1);
    }

    #[test]
    fn deeper_tree() {
        let tree = node_with_children(5,
            node_with_children(3, node(2), node(4)),
            node(6),
        );
        assert_eq!(kth_smallest(tree, 3), 4);
    }
}
