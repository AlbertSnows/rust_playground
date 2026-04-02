use crate::neetcode::linked_list::{ListNode, to_list, to_vec};

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let lists = vec![
            to_list(vec![1, 4, 5]),
            to_list(vec![1, 3, 4]),
            to_list(vec![2, 6]),
        ];
        assert_eq!(to_vec(merge_k_lists(lists)), vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn empty_lists() {
        assert_eq!(to_vec(merge_k_lists(vec![])), vec![]);
    }
}
