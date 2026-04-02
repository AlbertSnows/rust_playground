use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
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
        assert_eq!(level_order(tree), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn empty() {
        assert_eq!(level_order(None), Vec::<Vec<i32>>::new());
    }
}
