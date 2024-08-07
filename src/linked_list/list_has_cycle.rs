// Definition for singly-linked list.

pub struct Solution;

use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl Solution {
    pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
        let mut slow = &head;
        let mut fast = &head;

        while let Some(fast_node) = fast {
            if let Some(next_fast) = &fast_node.next {
                fast = &next_fast.next;
                slow = &slow.as_ref().unwrap().next;

                if slow == fast {
                    return true;
                }
            } else {
                break;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::list_has_cycle::Solution;

    #[test]
    fn test_has_cycle() {
        
    }
}