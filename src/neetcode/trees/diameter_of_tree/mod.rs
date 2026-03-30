use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let tree = node_with_children(1,
            node_with_children(2, node(4), node(5)),
            node(3),
        );
        assert_eq!(diameter_of_binary_tree(tree), 3);
    }

    #[test]
    fn two_nodes() {
        let tree = node_with_children(1, node(2), None);
        assert_eq!(diameter_of_binary_tree(tree), 1);
    }
}
