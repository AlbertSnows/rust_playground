use crate::neetcode::linked_list::{ListNode, to_list, to_vec};

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // 342 + 465 = 807
        let l1 = to_list(vec![2, 4, 3]);
        let l2 = to_list(vec![5, 6, 4]);
        assert_eq!(to_vec(add_two_numbers(l1, l2)), vec![7, 0, 8]);
    }

    #[test]
    fn with_carry() {
        // 999 + 1 = 1000
        let l1 = to_list(vec![9, 9, 9]);
        let l2 = to_list(vec![1]);
        assert_eq!(to_vec(add_two_numbers(l1, l2)), vec![0, 0, 0, 1]);
    }
}
