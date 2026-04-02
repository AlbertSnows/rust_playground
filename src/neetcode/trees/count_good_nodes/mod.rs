use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let tree = node_with_children(3,
            node_with_children(1, node(3), None),
            node_with_children(4, node(1), node(5)),
        );
        assert_eq!(good_nodes(tree), 4);
    }

    #[test]
    fn single_node() {
        assert_eq!(good_nodes(node(1)), 1);
    }
}
