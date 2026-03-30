use crate::neetcode::linked_list::{ListNode, to_list, to_vec};

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let list = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(to_vec(reverse_list(list)), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn two_elements() {
        let list = to_list(vec![1, 2]);
        assert_eq!(to_vec(reverse_list(list)), vec![2, 1]);
    }
}
