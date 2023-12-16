/*
    Need Review
    This is the first attempt I did to reverse lists in rust, I will try another attempt since I consider this one is not 
    concise enough.

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

        loop {            
            let mut left_node = match left_head {
                Some(left_node) => left_node,
                None => {
                    *prev = right_head;
                    break;
                }
            };

            let mut right_node = match right_head {
                Some(right_node) => right_node,
                None => {
                    *prev = Some(left_node);
                    break;
                }
            };

            if left_node.val < right_node.val {
                left_head = left_node.next.take();
                right_head = Some(right_node);
                *prev = Some(left_node);
            } else {
                right_head = right_node.next.take();
                left_head = Some(left_node);
                *prev = Some(right_node);
            }

            prev = &mut prev.as_mut().unwrap().next;
        }

        head
    }

    pub fn merge_two_lists_ex2(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut r = &mut l1;
        while l2.is_some() {
            if r.is_none() || l2.as_ref()?.val < r.as_ref()?.val {
                std::mem::swap(r, &mut l2);
            }
            r = &mut r.as_mut()?.next;
        }
        l1
    }

    pub fn merge_two_lists03(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(l1), Some(l2)) => {
                if l1.val >= l2.val {
                    Some(Box::new(ListNode {
                        val: l2.val,
                        next: Solution::merge_two_lists(Some(l1), l2.next)
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l1.val,
                        next: Solution::merge_two_lists(l1.next, Some(l2))
                    }))
                }
            }
        }
    }

    // By verified_tinker
    pub fn merge_two_list04(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(node_a), Some(node_b)) => {
                let (mut smaller, bigger) = Self::sort(node_a, node_b);
                smaller.next = Self::merge_two_lists(smaller.next, Some(bigger));
                Some(smaller)
            }
            (node, None) | (None, node) => node,
            // The above will also match on `(None, None)` and return `None`, as it should.
        }
    }

    const fn sort(list1: Box<ListNode>, list2: Box<ListNode>) -> (Box<ListNode>, Box<ListNode>) {
        if list1.val < list2.val {
            (list1, list2)
        } else {
            (list2, list1)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::merge_linked_list_attempt_02::{Solution, ListNode};
    
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

        assert_eq!(res.to_string(), "1,1,2,3,3,4,4");
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

    #[test]
    fn test_merge06() {
        let list1 = Some(Box::new(ListNode { 
            val: 5,
            next: None
        }));

        let list2 = Some(Box::new(ListNode { 
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None
                }))
            }))
        }));

        let res = Solution::merge_two_lists(list1, list2).unwrap();

        assert_eq!(res.to_string(), "1,2,4,5");
    }
}