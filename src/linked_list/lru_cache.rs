use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Node {
    key: i32,
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    pub fn new(capacity: i32) -> Self {
        let head = Rc::new(RefCell::new(Node {
            key: 0,
            value: 0,
            next: None,
            prev: None,
        }));

        let tail = Rc::new(RefCell::new(Node {
            key: 0,
            value: 0,
            next: None,
            prev: None,
        }));

        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::clone(&head));

        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            head,
            tail,
        }
    }
    
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node_rc) = self.map.get(&key) {
            let value = node_rc.borrow().value;
            self.move_to_head(Rc::clone(node_rc));

            return value;
        }

        -1
    }
    
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node_rc) = self.map.get(&key) {
            node_rc.borrow_mut().value = value;

            self.move_to_head(Rc::clone(node_rc));

            return;
        }

        if self.map.len() == self.capacity {
            self.pop_tail();
        }

        let new_node = Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }));

        self.map.insert(key, Rc::clone(&new_node));
        self.add_node(Rc::clone(&new_node));
    }

    fn add_node(&mut self, node: Rc<RefCell<Node>>) {
        let mut head_next = self.head.borrow_mut();
        let head_next_ref = head_next.next.take().unwrap();

        node.borrow_mut().next = Some(Rc::clone(&head_next_ref));
        node.borrow_mut().prev = Some(Rc::clone(&self.head));

        head_next_ref.borrow_mut().prev = Some(Rc::clone(&node));
        head_next.next = Some(node);
    }

    fn remove_node(&mut self, node: Rc<RefCell<Node>>) {
        let prev_node = node.borrow_mut().prev.take().unwrap();
        let next_node = node.borrow_mut().next.take().unwrap();

        prev_node.borrow_mut().next = Some(Rc::clone(&next_node));
        next_node.borrow_mut().prev = Some(Rc::clone(&prev_node));
    }

    fn move_to_head(&mut self, node: Rc<RefCell<Node>>) {
        self.remove_node(Rc::clone(&node));
        self.add_node(node);
    }

    fn pop_tail(&mut self) {
        let tail_prev_temp = self.tail.borrow_mut().prev.take();

        if let Some(tail_prev) = tail_prev_temp {
            self.remove_node(Rc::clone(&tail_prev));
            let key = tail_prev.borrow().key;
            self.map.remove(&key);
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::linked_list::lru_cache::*;

    #[test]
    fn test_cache() {
        let mut cache = LRUCache::new(2);

        cache.put(1, 1);
        cache.put(2, 2);

        assert_eq!(cache.get(1), 1);

        cache.put(3, 3); // evicts key 2

        assert_eq!(cache.get(2), -1);

        cache.put(4, 4);

        assert_eq!(cache.get(1), -1);

        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
