use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let tree = node_with_children(2, node(1), node(3));
        assert_eq!(is_valid_bst(tree), true);
    }

    #[test]
    fn invalid() {
        let tree = node_with_children(5,
            node(1),
            node_with_children(4, node(3), node(6)),
        );
        assert_eq!(is_valid_bst(tree), false);
    }
}
