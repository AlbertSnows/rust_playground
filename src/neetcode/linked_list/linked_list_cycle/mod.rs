use crate::neetcode::linked_list::ListNode;

pub fn has_cycle(_head: Option<Box<ListNode>>) -> bool {
    // Note: cycle detection with Box<ListNode> is contrived; real impl uses raw pointers.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neetcode::linked_list::to_list;

    #[test]
    fn no_cycle() {
        let list = to_list(vec![1, 2, 3]);
        assert_eq!(has_cycle(list), false);
    }

    #[test]
    fn empty_list() {
        assert_eq!(has_cycle(None), false);
    }
}
