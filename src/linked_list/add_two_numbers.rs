// Definition for singly-linked list.

pub struct Solution;

use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    // Helper function to create a linked list from a vector of values
    pub fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;

        for &value in values.iter().rev() {
            let new_node = Box::new(ListNode {
                val: value,
                next: current,
            });

            current = Some(new_node);
        }

        current
    }
}

impl fmt::Display for ListNode  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[{}, {self:p}]", self.val)?;

        let mut node = self.next.clone();

        while let Some(n) = node {
            writeln!(
                f,
                "[{}, {n:p}]",
                n.val)?;

            node = n.next;
        }

        Ok(())
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current_l1 = l1;
        let mut current_l2 = l2;
        let mut sum_head = ListNode::new(-1);
        let mut sum_prev = &mut sum_head;

        let mut carry = 0;

        while let Some(mut node_l1) = current_l1 {
            let mut sum = carry + node_l1.val;

            if let Some(mut node_l2) = current_l2 {
                sum += node_l2.val;
                current_l2 = node_l2.next.take();
            }

            carry = sum / 10;
            
            sum_prev.next = Some(Box::new(ListNode::new(sum % 10)));
            sum_prev = sum_prev.next.as_mut().unwrap();
        
            current_l1 = node_l1.next.take();
        }

        while let Some(mut node_l2) = current_l2 {
            let sum = carry + node_l2.val;

            carry = sum / 10;

            sum_prev.next = Some(Box::new(ListNode::new(sum % 10)));
            sum_prev = sum_prev.next.as_mut().unwrap();

            current_l2 =  node_l2.next.take();
        }

        if carry > 0 {
            sum_prev.next = Some(Box::new(ListNode::new(carry)));
        }

        sum_head.next
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::add_two_numbers::{ListNode, Solution};

    #[test]
    fn test_01() {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);

        println!("{} + {}", l1.as_ref().unwrap(), l2.as_ref().unwrap());

        let l3 = Solution::add_two_numbers(l1, l2);


        println!("{}", l3.unwrap())
    }
}
