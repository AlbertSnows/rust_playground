use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let tree = node_with_children(3,
            node_with_children(9, None, None),
            node_with_children(20, node(15), node(7)),
        );
        assert_eq!(max_depth(tree), 3);
    }

    #[test]
    fn empty() {
        assert_eq!(max_depth(None), 0);
    }
}
