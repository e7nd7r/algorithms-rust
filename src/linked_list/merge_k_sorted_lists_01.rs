use std::cmp::{Reverse, Ordering};
use std::collections::BinaryHeap;

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
  pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
      if lists.is_empty() {
        return  None;
      }

      let mut head = None;
      let mut tail = &mut head;

      let mut heap = lists
        .into_iter()
        .filter(Option::is_some)
        .map(Reverse)
        .collect::<BinaryHeap<_>>();

      while let Some(Reverse(mut curr)) = heap.pop() {
          let next = curr
            .as_mut()
            .and_then(|curr| curr.next.take());

          if next.is_some() {
            heap.push(Reverse(next))
          }

          *tail = curr;

          tail = &mut tail.as_mut().unwrap().next;
      }

      head
  }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
  fn cmp(&self, other: &Self) -> Ordering {
    self.val.cmp(&other.val)
  }
}