use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let tree = node_with_children(1,
            node_with_children(2, None, node(5)),
            node_with_children(3, None, node(4)),
        );
        assert_eq!(right_side_view(tree), vec![1, 3, 4]);
    }

    #[test]
    fn empty() {
        assert_eq!(right_side_view(None), vec![] as Vec<i32>);
    }
}
