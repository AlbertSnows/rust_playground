use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let tree = node_with_children(1, node(2), node(3));
        assert_eq!(max_path_sum(tree), 6);
    }

    #[test]
    fn negative_nodes() {
        let tree = node_with_children(-10,
            node(9),
            node_with_children(20, node(15), node(7)),
        );
        assert_eq!(max_path_sum(tree), 42);
    }
}
