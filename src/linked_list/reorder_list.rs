// Definition for singly-linked list.

use std::{ops::Deref, hint::unreachable_unchecked};

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn to_string(&self) -> String {
        let mut node = Some(self);
        let mut nodes = Vec::new();

        while let Some(n) = node {
            nodes.push(n.val.to_string());
            node = n.next.as_deref();
        }

        nodes.join(",")
    }
}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut stack = Vec::with_capacity(10);
        
        let mut curr: Option<Box<ListNode>> = head.take();

        while let Some(mut node) = curr {
            curr = node.next.take();
            stack.push(Some(node));
        }

        let mut i = 0;
        let mut j = stack.len() - 1;
        let mut c = 0;
        let mut ptr = head;

        while i <= j {
            if c % 2 == 0 {
                *ptr = stack.remove(0);
                i += 1;
            } else {
                *ptr = stack.pop().unwrap();
                j -= 1;
            }
            c += 1;
            ptr = &mut ptr.as_mut().unwrap().next;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::reorder_list::*;

    #[test]
    fn test_reorder_list01() {
        let mut list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }));
        
        Solution::reorder_list(&mut list);

        assert_eq!(list.unwrap().to_string(), "1,4,2,3")
    }

    #[test]
    fn test_reorder_list02() {
        let mut list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None }))
                    })),
                })),
            })),
        }));
        
        Solution::reorder_list(&mut list);

        assert_eq!(list.unwrap().to_string(), "1,5,2,4,3")
    }
}
