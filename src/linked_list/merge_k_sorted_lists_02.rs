use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        
        for list in lists {
            if let Some(node) = list {
                heap.push(node);
            }
        }

        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        while let Some(mut node) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(next);
            }

            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}
