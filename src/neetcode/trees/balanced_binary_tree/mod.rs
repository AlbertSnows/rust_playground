use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced() {
        let tree = node_with_children(3,
            node_with_children(9, None, None),
            node_with_children(20, node(15), node(7)),
        );
        assert_eq!(is_balanced(tree), true);
    }

    #[test]
    fn unbalanced() {
        let tree = node_with_children(1,
            node_with_children(2,
                node_with_children(3, node(4), node(4)),
                node(3),
            ),
            node(2),
        );
        assert_eq!(is_balanced(tree), false);
    }
}
