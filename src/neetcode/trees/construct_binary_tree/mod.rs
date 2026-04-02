use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let result = build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        let expected = node_with_children(3,
            node(9),
            node_with_children(20, node(15), node(7)),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn single_node() {
        assert_eq!(build_tree(vec![1], vec![1]), node(1));
    }
}
