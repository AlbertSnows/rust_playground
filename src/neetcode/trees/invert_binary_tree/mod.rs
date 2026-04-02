use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        //       4              4
        //      / \    =>      / \
        //     2   7          7   2
        //    / \ / \        / \ / \
        //   1  3 6  9      9  6 3  1
        let tree = node_with_children(4,
            node_with_children(2, node(1), node(3)),
            node_with_children(7, node(6), node(9)),
        );
        let result = invert_tree(tree);
        let expected = node_with_children(4,
            node_with_children(7, node(9), node(6)),
            node_with_children(2, node(3), node(1)),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(invert_tree(None), None);
    }
}
