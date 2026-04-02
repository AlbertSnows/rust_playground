use crate::neetcode::linked_list::{ListNode, to_list, to_vec};

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut head = to_list(vec![1, 2, 3, 4]);
        reorder_list(&mut head);
        assert_eq!(to_vec(head), vec![1, 4, 2, 3]);
    }

    #[test]
    fn five_elements() {
        let mut head = to_list(vec![1, 2, 3, 4, 5]);
        reorder_list(&mut head);
        assert_eq!(to_vec(head), vec![1, 5, 2, 4, 3]);
    }
}
