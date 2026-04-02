use crate::neetcode::linked_list::{ListNode, to_list, to_vec};

pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let l1 = to_list(vec![1, 2, 4]);
        let l2 = to_list(vec![1, 3, 4]);
        assert_eq!(to_vec(merge_two_lists(l1, l2)), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn one_empty() {
        let l1 = to_list(vec![]);
        let l2 = to_list(vec![0]);
        assert_eq!(to_vec(merge_two_lists(l1, l2)), vec![0]);
    }
}
