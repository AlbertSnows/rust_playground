use crate::neetcode::linked_list::{ListNode, to_list, to_vec};

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let list = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(to_vec(remove_nth_from_end(list, 2)), vec![1, 2, 3, 5]);
    }

    #[test]
    fn single_element() {
        let list = to_list(vec![1]);
        assert_eq!(to_vec(remove_nth_from_end(list, 1)), vec![]);
    }
}
