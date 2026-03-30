pub mod reverse_linked_list;
pub mod merge_two_sorted_lists;
pub mod reorder_list;
pub mod remove_nth_from_end;
pub mod linked_list_cycle;
pub mod add_two_numbers;
pub mod find_duplicate;
pub mod lru_cache;
pub mod merge_k_sorted_lists;
pub mod reverse_k_group;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn to_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = head;
        head = Some(node);
    }
    head
}

pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut cur = head;
    while let Some(node) = cur {
        result.push(node.val);
        cur = node.next;
    }
    result
}
