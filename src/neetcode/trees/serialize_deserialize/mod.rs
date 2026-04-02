use std::rc::Rc;
use std::cell::RefCell;
use crate::neetcode::trees::{TreeNode, node, node_with_children};

pub fn serialize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    Default::default()
}

pub fn deserialize(data: String) -> Option<Rc<RefCell<TreeNode>>> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let tree = node_with_children(1, node(2), node_with_children(3, node(4), node(5)));
        let serialized = serialize(tree.clone());
        assert_eq!(deserialize(serialized), tree);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(serialize(None), "null".to_string());
        assert_eq!(deserialize("null".to_string()), None);
    }
}
