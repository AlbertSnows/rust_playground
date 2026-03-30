use crate::neetcode::linked_list::{ListNode, to_list, to_vec};

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let list = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(to_vec(reverse_k_group(list, 2)), vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn k_equals_three() {
        let list = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(to_vec(reverse_k_group(list, 3)), vec![3, 2, 1, 4, 5]);
    }
}
