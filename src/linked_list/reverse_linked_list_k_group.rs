use std::rc::Rc;
use std::cell::RefCell;

pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

pub struct Solution;

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
        }
    }
}

impl Solution {
    pub fn reverse_list_k_group(head: Option<Rc<RefCell<ListNode>>>, k: i32) -> Option<Rc<RefCell<ListNode>>> {
        let dummy: Rc<RefCell<ListNode>> = Rc::new(RefCell::new(ListNode { val: 0, next: head }));
        let mut groupPrev = dummy;

        loop {
            let kth = Solution::get_kth_node(Rc::clone(&groupPrev), k);
        }

        None
    }

    fn get_kth_node(curr: Rc<RefCell<ListNode>>, k: i32) -> Option<Rc<RefCell<ListNode>>> {
        let mut k = k;
        let mut curr = Some(&curr);

        while let Some(node_rc) = curr {
            if k == 0  {
                break;
            }

            k -= 1;
            
        }

        Some(Rc::clone(curr.unwrap()))
    }
}
