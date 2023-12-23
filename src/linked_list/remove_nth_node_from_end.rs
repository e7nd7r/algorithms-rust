/*
    Based my solution in the following solution since I was stuck with the borrow checking.

    https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/3463071/rust-solution-without-any-unwarp-or-unsafe/
*/

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn to_string(&self) -> String {
        let mut node = Some(self);
        let mut nodes = Vec::new();

        while let Some(n) = node {
            //nodes.push(format!("[{}, {:p}]", n.val.to_string(), n));
            nodes.push(n.val.to_string());
            node = n.next.as_deref();
        }

        nodes.join(",")
    }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let (mut fast, mut slow) = (dummy.clone(), dummy.as_mut());
        let mut d = 0;

        while let Some(node) = fast.next {
            if d < n {
                d += 1;
            } else {
                slow = match slow.next.as_mut() {
                    None => return None,
                    Some(s) => s,
                };
            }

            fast = node;
        }

        let remove = match slow.next.as_mut() {
            Some(s) => s,
            None => return None,
        };

        slow.next = remove.next.take();

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::remove_nth_node_from_end::{ListNode, Solution};

    #[test]
    fn test_remove_nth_from_end() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
       
        //println!("Addresses original");
        //println!("{}", list.as_ref().unwrap().to_string());

        let res = Solution::remove_nth_from_end(list, 2);

        //println!("Addresses response");
        //println!("{}", res.as_ref().unwrap().to_string());
        
        assert_eq!(
            res.as_ref().unwrap().to_string(),
            "1,2,3,5"
        );
    }
}
