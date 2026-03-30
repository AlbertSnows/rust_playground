use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        //       6
        //      / \
        //     2   8
        //    / \ / \
        //   0  4 7  9
        let tree = node_with_children(6,
            node_with_children(2, node(0), node_with_children(4, node(3), node(5))),
            node_with_children(8, node(7), node(9)),
        );
        assert_eq!(lowest_common_ancestor(tree, 2, 8), 6);
    }

    #[test]
    fn ancestor_is_node() {
        let tree = node_with_children(6,
            node_with_children(2, node(0), node_with_children(4, node(3), node(5))),
            node_with_children(8, node(7), node(9)),
        );
        assert_eq!(lowest_common_ancestor(tree, 2, 4), 2);
    }
}
