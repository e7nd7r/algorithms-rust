struct Solution {
}

// Definition for singly-linked list.
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;
 
        while let Some(mut node) = curr.take() {
            let next = node.next;
            node.next = prev;
            prev = Some(node);
            curr = next
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::reverse_linked_list::{Solution, ListNode};

    #[test]
    fn test_reverse_linked_list01() {
        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(2);
        let mut node3 = ListNode::new(3);
        let node4 =  ListNode::new(4);

        node3.next = Some(Box::new(node4));
        node2.next = Some(Box::new(node3));
        node1.next = Some(Box::new(node2));

        print!("{}", node1.to_string());

        let res = Solution::reverse_list(Some(Box::new(node1))).unwrap();

        assert_eq!(res.to_string(), "4,3,2,1");
    }
}
