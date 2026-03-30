pub mod invert_binary_tree;
pub mod max_depth;
pub mod diameter_of_tree;
pub mod balanced_binary_tree;
pub mod same_tree;
pub mod subtree_of_another;
pub mod lca_bst;
pub mod level_order_traversal;
pub mod right_side_view;
pub mod count_good_nodes;
pub mod validate_bst;
pub mod kth_smallest_bst;
pub mod construct_binary_tree;
pub mod max_path_sum;
pub mod serialize_deserialize;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

pub fn node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

pub fn node_with_children(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let n = Rc::new(RefCell::new(TreeNode::new(val)));
    n.borrow_mut().left = left;
    n.borrow_mut().right = right;
    Some(n)
}
