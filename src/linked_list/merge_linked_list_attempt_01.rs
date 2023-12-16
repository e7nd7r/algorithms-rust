/*
    This is the first attempt I did to reverse lists in rust, I will try another attempt since I consider this one is not 
    concise enough. This solution doesn't work.

    Learnings: 
        - It's hard to manage ownership while using pattern matching. Probably I need to avoid it.
        - <option>.take = mem::replace(<option>, None)

*/

pub struct Solution {
}

// Definition for singly-linked list.
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut left_head = list1;
        let mut right_head = list2;
        let mut head = None;
        let mut prev = &mut head;

        while let Some(mut left_node) = left_head {
            // here we are partially moving the value of left_node.next to next_left.
            // by using left_node.next.take() we replace the value of next with None first
            // then we move the previous value, so it won't complaing when reassining the value belo
            let next_left = left_node.next.take();
        
            prev = match right_head.take() {
                Some(mut right_node) => {
                    if right_node.val > left_node.val {
                        right_head = right_node.next.take();
                        left_node.next = Some(right_node);
                        *prev = Some(left_node);
                    } else {
                        // Same than left_node.next we need to use take function to avoid changing the ownership of
                        // the original value.
                        right_head = right_node.next.take();
                        // here we are trying to set `left.next` to right node but since left_node.next would be owned by next_left 
                        // it might happend that the value won't be available. So the solution it's replace the current space in the memory
                        // with something else and the yield the ownership of the original value.
                        right_node.next = Some(left_node);
                        *prev = Some(right_node);
                    }

                    &mut prev.as_mut().unwrap().next
                },
                None => {
                    *prev = Some(left_node);
                    prev
                },
            };

            left_head = next_left;
            prev = &mut prev.as_mut().unwrap().next;
        }

        if !right_head.is_none() {
            *prev = right_head;
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::merge_linked_list_attempt_01::{Solution, ListNode};
    
    #[test]
    fn test_merge01() {
        let list1 = Some(Box::new(ListNode { 
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        }));

        let list2 = Some(Box::new(ListNode { 
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None
                }))
            }))
        }));

        let res = Solution::merge_two_lists(list1, list2).unwrap();

        assert_eq!(res.to_string(), "1,1,2,3,3,4");
    }

    #[test]
    fn test_merge02() {
        let list1 = Some(Box::new(ListNode { 
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        }));

        let list2 = None;
        let res = Solution::merge_two_lists(list1, list2).unwrap();

        assert_eq!(res.to_string(), "1,2,3,4");
    }

    #[test]
    fn test_merge03() {
        let list1 = None;

        let list2 = Some(Box::new(ListNode { 
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None
                }))
            }))
        }));

        let res = Solution::merge_two_lists(list1, list2).unwrap();

        assert_eq!(res.to_string(), "1,3,4");
    }

    #[test]
    fn test_merge04() {
        let list1 = None;
        let list2 = None;

        let res = Solution::merge_two_lists(list1, list2);

        assert_eq!(res, None);
    }

    #[test]
    fn test_merge05() {
        let list1 = Some(Box::new(ListNode { 
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None
                }))
            }))
        }));

        let list2 = Some(Box::new(ListNode { 
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None
                }))
            }))
        }));

        let res = Solution::merge_two_lists(list1, list2).unwrap();

        assert_eq!(res.to_string(), "1,1,2,3,4,4");
    }
}