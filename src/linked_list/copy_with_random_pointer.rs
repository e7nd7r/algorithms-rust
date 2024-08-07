/*
Notes:

Ownership Rules
First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

Each value in Rust has an owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.

https://whiztal.io/rust-tips-rc-box-arc-cell-refcell-mutex/

Most types containing pointers to other data are not Copy, since there is additional data elsewhere, and simply copying the stack representation may accidentally share ownership of that data in an unsafe manner.

&T and raw pointers are Copy. Even though they do point to further data, they do not “own” that data.

&mut T is not Copy because mutable aliases cannot be shared, and &mut T “owns” the data it points to somewhat since it can mutate.

smart pointer 

Weak<T>. This is a non-owning, but also non-borrowed, smart pointer. This is useful for when one wants cyclic data structures and other things.
Box<T> is for single ownership.
Rc<T> is for multiple ownership.
Arc<T> is for multiple ownership, but threadsafe.
Cell<T> is for "interior mutability" for Copy types; that is, when you need to mutate something behind a &T.

https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/
*/

use std::{cell::RefCell, collections::HashMap, f64::consts, fmt, rc::Rc};

struct Solution {
}

type Link = Option<Rc<RefCell<ListNode>>>;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Link,
  pub random: Link
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            random: None,
            val
        }
    }

    pub fn to_string(&self) -> String {
        let mut values = Vec::new();

        self._to_string(&mut values);
    
        values.join(",")
    }

    pub(self) fn _to_string(&self, values: &mut Vec<String>) {
        values.push(self.val.to_string());

        if self.next.is_some() {
            self.next.as_ref().unwrap().as_ref().borrow()._to_string(values);
        }
    }
}

impl fmt::Display for ListNode  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let random = self.random.clone().map_or(
            "null".to_string(), 
            |x| x.clone().borrow().val.to_string());
        
        writeln!(f, "[{}, {self:p}, {}]", self.val, random)?;

        let mut node = self.next.clone();

        while let Some(n) = node {
            let n_borrowed: std::cell::Ref<'_, ListNode> = n.as_ref().borrow();
            let random = n_borrowed.random.clone().map_or(
                "null".to_string(), 
                |x| x.clone().borrow().val.to_string());

            let ref_cell = n.as_ref();
            writeln!(
                f,
                "[{}, {ref_cell:p}, {}]",
                n_borrowed.val,
                random)?;

            node = n_borrowed.next.clone();
        }

        Ok(())
    }
}

impl Solution {
    fn create_node_map(head: &Link) -> HashMap<*const ListNode, Rc<RefCell<ListNode>>> {
        let mut node_map = HashMap::new();
        let mut current = head.as_ref().map(Rc::clone);

        while let Some(node) = current {
            let node_ref = node.borrow();
            let copied_node = Rc::new(RefCell::new(ListNode::new(node_ref.val)));
            node_map.insert(&*node_ref as *const _, Rc::clone(&copied_node));

            current = node_ref.next.as_ref().map(Rc::clone);
        }

        node_map
    }

    fn update_next_pointer(
        node_ref: &ListNode,
        copied_node: &Rc<RefCell<ListNode>>,
        node_map: &HashMap<*const ListNode, Rc<RefCell<ListNode>>>
    ) {
        if let Some(next_node) = node_ref.next.as_ref() {
            if let Some(copied_next) = node_map.get(&(&*next_node.borrow() as *const _)) {
                copied_node.borrow_mut().next = Some(Rc::clone(copied_next));
            }
        } 
    }
    
    fn update_random_pointer(
        node_ref: &ListNode,
        copied_node: &Rc<RefCell<ListNode>>,
        node_map: &HashMap<*const ListNode, Rc<RefCell<ListNode>>>
    ) {
        if let Some(random_node) = node_ref.random.as_ref() {
            if let Some(copied_random) = node_map.get(&(&*random_node.borrow() as *const _)) {
                copied_node.borrow_mut().random = Some(Rc::clone(copied_random))
            }
        }
    }

    fn update_pointers(
        node_map: &HashMap<*const ListNode, Rc<RefCell<ListNode>>>,
        head: &Link
    ){
        let mut current = head.as_ref().map(Rc::clone);

        while let Some(node) = current {
            // Get node reference
            let node_ref = node.borrow();

            // Get copied value from node reference.
            if let Some(copied_node) = node_map.get(&(&*node_ref as *const _)) {
                Solution::update_next_pointer(&node_ref, copied_node, &node_map);
                Solution::update_random_pointer(&node_ref, copied_node, &node_map);
            }

            current = node_ref.next.as_ref().map(Rc::clone);
        }
    }

    #[allow(dead_code)]
    fn copy(head: Link) -> Link {
        let node_map = Solution::create_node_map(&head); 

        Solution::update_pointers(&node_map, &head);
        
        head.and_then(|node| node_map.get(&(&*node.borrow() as *const _)).cloned())
    }

    
}

#[cfg(test)]
mod tests {
    use std::{rc::Rc, cell::RefCell};

    use crate::linked_list::copy_with_random_pointer::{ListNode, Solution};

    #[test]
    fn test_copy() {
        let node1 = Rc::new(RefCell::new(ListNode::new(1)));
        let node2 = Rc::new(RefCell::new(ListNode::new(2)));
        let node3 = Rc::new(RefCell::new(ListNode::new(3)));
        let node4 = Rc::new(RefCell::new(ListNode::new(4)));
        
        node3.as_ref().borrow_mut().next = Some(node4.clone());
        node2.as_ref().borrow_mut().next = Some(node3.clone());
        node1.as_ref().borrow_mut().next = Some(node2.clone());

        node1.as_ref().borrow_mut().random = Some(node1.clone());

        println!("{}", node1.as_ref().borrow().to_string());

        let new_head = Solution::copy(Some(node1.clone()));

        println!("{}", new_head.unwrap().clone().borrow());
    }
}