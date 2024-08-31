use std::collections::VecDeque;
use std::{fmt, fs};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fs::File;

pub struct Solution;

pub struct Person {
    pub name: String,
}

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct TreeNode {
    pub value: i32,
    pub children: Vec<Rc<TreeNode>>,
}

pub struct RcTree {
    pub root: Rc<TreeNode>,
}

#[derive(Clone)]
pub struct RefCellTreeNode {
    pub value: i32,
    pub children: Vec<RefCell<RefCellTreeNode>>,
}

pub struct RefCellTree {
    pub root: RefCell<RefCellTreeNode>,
}

pub struct RcRefCellTreeNode {
    pub value: i32,
    pub acc: i32,
    pub children: Vec<Rc<RefCell<RcRefCellTreeNode>>>,
}

pub struct RcRefCellTree {
    pub root: Rc<RefCell<RcRefCellTreeNode>>,
}

pub enum Operation {
    Sum,
    Mult,
    NoOp
}

pub struct WeakTreeNode {
    pub value: i32,
    pub acc: i32,
    pub operation: Operation,
    pub parent: Weak<RefCell<WeakTreeNode>>,
    pub children: Vec<Rc<RefCell<WeakTreeNode>>>,
}

pub struct WeakTree {
    pub root: Rc::<RefCell<WeakTreeNode>>,
}

pub struct FileSmartPointer {
    pub filename: String,
    pub file: File,
}

impl WeakTreeNode {
    pub fn new(value: i32, acc: i32, operation: Operation) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(WeakTreeNode {
            value,
            acc,
            operation,
            parent: Weak::new(),
            children: vec![],
        }))
    }

    pub fn add_child(parent: &Rc<RefCell<WeakTreeNode>>, child: Rc<RefCell<WeakTreeNode>>) {
        child.borrow_mut().parent = Rc::downgrade(parent);
        parent.borrow_mut().children.push(child);
    }
}

impl ListNode {
    pub fn from(arr: Vec<i32>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode {
            val: 0,
            next: None
        };

        let mut curr = &mut dummy;

        if arr.len() == 0 {
            return None;
        }

        for elem in arr {
            let new_node = ListNode {
                val: elem,
                next: None
            };

            curr.next = Some(Box::new(new_node));

            curr = curr.next.as_mut().unwrap();
        }

        return dummy.next;
    }

    pub fn to(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = head;

        // Traverse the linked list and collect values into the vector
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next;
        }

        vec
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)?;

        if let Some(ref next_node) = self.next {
            write!(f, "-> {}", next_node)?;
        }

        Ok(())
    }
}

impl fmt::Display for RcTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue = VecDeque::new();
        
        queue.push_back(Rc::clone(&self.root));

        while let Some(node_rc) = queue.pop_front(){
            if queue.is_empty() && node_rc.as_ref().children.is_empty() {
                write!(f, "{} ", node_rc.as_ref().value)?;
            } else {
                write!(f, "{} -> ", node_rc.as_ref().value)?;
            }
            
            for node in &node_rc.as_ref().children {
                queue.push_back(Rc::clone(node));
            }
        }
        
        Ok(())
    }
}

impl fmt:: Display for RefCellTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue = VecDeque::new();
        
        queue.push_back(self.root.as_ptr());

        while let Some(node_rc) = queue.pop_front(){
            unsafe {
                if queue.is_empty() && (*node_rc).children.is_empty() {
                    write!(f, "{} ", (*node_rc).value)?;
                } else {
                    write!(f, "{} -> ", (*node_rc).value)?;
                }
                
                for node in &(*node_rc).children {
                    queue.push_back(node.as_ptr());
                }
            }
        }
        
        Ok(())
    }
}

// The following code is a reimplementation of the code above using clone to satify
// borrowng checking.
// impl fmt:: Display for RefCellTree {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut queue = VecDeque::new();
//         queue.push_back(self.root.borrow().clone());

//         while let Some(node) = queue.pop_front() {
//             if queue.is_empty() && node.children.is_empty() {
//                 write!(f, "{} ", node.value)?;
//             } else {
//                 write!(f, "{} -> ", node.value)?;
//             }

//             for child in &node.children {
//                 queue.push_back(child.borrow().clone());
//             }
//         }
        
//         Ok(())
//     }
// }

impl fmt::Display for RcRefCellTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue = VecDeque::new();
        
        queue.push_back(Rc::clone(&self.root));

        while let Some(ref node_rc) = queue.pop_front(){
            let node_borrowed = node_rc.as_ref().borrow();

            if queue.is_empty() && node_borrowed.children.is_empty() {
                write!(f, "({},{}) ", node_borrowed.value, node_borrowed.acc)?;
            } else {
                write!(f, "({},{}) -> ", node_borrowed.value, node_borrowed.acc)?;
            }
            
            for node in &node_borrowed.children {
                queue.push_back(Rc::clone(node));
            }
        }
        
        Ok(())
    }
}

impl fmt::Display for WeakTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue = VecDeque::new();
        
        queue.push_back(Rc::clone(&self.root));

        while let Some(ref node_rc) = queue.pop_front(){
            let node_borrowed = node_rc.as_ref().borrow();

            if queue.is_empty() && node_borrowed.children.is_empty() {
                write!(f, "({},{}) ", node_borrowed.value, node_borrowed.acc)?;
            } else {
                write!(f, "({},{}) -> ", node_borrowed.value, node_borrowed.acc)?;
            }
            
            for node in &node_borrowed.children {
                queue.push_back(Rc::clone(node));
            }
        }
        
        Ok(())
    }
}

impl FileSmartPointer {
    pub fn new(filename: &str) -> Self {
        let file = File::create(filename).expect("could not create file.");

        FileSmartPointer {
            filename: filename.to_string(),
            file
        }
    }
}

impl Drop for FileSmartPointer {
    fn drop(&mut self) {
        fs::remove_file(&self.filename).expect("Could not delete file!");

        println!("Dropping file pointer!");
    }
}

impl Solution {
    pub fn len(str: String) -> usize { 
        str.len()
    }
    pub fn inmmutable_len(str: &String) -> usize{
        // Uncommenting the following line will fail since str it's a inmutable reference.
        //str.push_str(" Marino");

        str.len()
    }

    pub fn append_text(str: &mut String) {
        str.push_str(" Marino");
    }

    pub fn person_greating(person: Person) {
        println!("hello {}", person.name)
    }

    pub fn person_greating_immutable(person: &Person) {
        println!("hello {}", person.name)
    }

    pub fn get_longest_str<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        if str1.len() > str2.len() {
            return str1;
        }

        str2
    }
    
    pub fn execute_cost(tree: &RcRefCellTree) {
        fn execute_cost_rec(node: &Rc<RefCell<RcRefCellTreeNode>>) -> i32 {
            
            let mut borrowed_node = node.as_ref().borrow_mut();

            if borrowed_node.children.is_empty() {
                borrowed_node.acc += borrowed_node.value;

                return borrowed_node.acc;
            }

            borrowed_node.acc += borrowed_node.value +
                    borrowed_node.children
                    .iter()
                    .fold(0,|acc, rc_node| {
                        acc + execute_cost_rec(rc_node)
                    });

            return borrowed_node.acc;
        }

        execute_cost_rec(&tree.root);
    }

    pub fn calculate(tree: &WeakTree) {
        fn calculate_rec(node: &Rc<RefCell<WeakTreeNode>>) -> i32 {
            
            let mut borrowed_node = node.as_ref().borrow_mut();


            if borrowed_node.children.is_empty() {
                borrowed_node.acc = borrowed_node.value;
                return borrowed_node.acc;
            }

            let neutral_val = match borrowed_node.operation {
                Operation::Sum => 0,
                Operation::Mult => 1,
                Operation::NoOp => panic!("No leaf nodes should define an ")
            };

            let children_result =
                    borrowed_node.children
                    .iter()
                    .fold(neutral_val,|acc, rc_node| {
                        match borrowed_node.operation {
                            Operation::Sum => acc + calculate_rec(rc_node),
                            Operation::Mult => acc * calculate_rec(rc_node),
                            Operation::NoOp => borrowed_node.value,
                        }
                    });

            borrowed_node.acc = match borrowed_node.operation {
                Operation::Sum => children_result + borrowed_node.value,
                Operation::Mult => children_result * borrowed_node.value,
                Operation::NoOp => borrowed_node.acc,
            };

            return borrowed_node.acc;
        }

        calculate_rec(&tree.root);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use crate::learning_series::serie1_ownership::*;
    
    #[test]
    fn exercise_01() {
        let s = String::from("hello world");       
        assert_eq!(Solution::len(s), 11);

        // Uncommenting the following line will cause a compile-time error
        // because `my_string` has been moved and is no longer valid here.
        // println!("The string is: {}", s);
    }

    #[test]
    fn exercise_02() {
        let mut s = String::from("hello world");

        assert_eq!(Solution::inmmutable_len(&s), 11);

        // The following line works since inmutable_len is borrowing the value.
        println!("The string is: {}", s);
        
        let _x = &mut s;

        Solution::append_text(&mut s);
        
        // Uncommenting the following line will fail since we it's borrowing the value twice as mutable.
        // Notice x was declared above the function append_text call so that the scope is larger than
        // this function.
        // _x.push_str("string");
    }

    #[test]
    fn exercise_03() {
        let mut person = Person {
            name: String::from("e7nd7r"),
        };

        let _y = &mut person;

        // This work because it's borrowing person as inmutable.
        Solution::person_greating_immutable(&person);
        
        // uncommenting next line fail since person was already borrow as inmutable. 
        // (one mutable or multipel inmutable)
        // _y.name.push_str(" o5r");

        Solution::person_greating(person);

        // Uncommenting the following line will fail since 
        // ownership was transfered above.
        // println!("hello again {}", person.name);
    }

    #[test]
    fn exercise_04() {
        assert_eq!(Solution::get_longest_str("abc", "abcd"), "abcd");
    }

    #[test]
    fn exercise_05() {
        let mut head = ListNode::from(vec![1, 2,3]);

        let mut first = head.take();
        let mut second = first.as_mut().unwrap().next.take();
        let mut third = second.as_mut().unwrap().next.take();

        second.as_mut().unwrap().next = first;
        third.as_mut().unwrap().next = second;
    }

    #[test]
    fn exercise_06() {
        let node6 = Rc::new(TreeNode {
            value: 6,
            children: vec![],
        });

        let node5 =  Rc::new(TreeNode {
            value: 5,
            children: vec![],
        });

        let node4 =  Rc::new(TreeNode {
            value: 4,
            children: vec![],
        });

        let node3 =  Rc::new(TreeNode {
            value: 3,
            children: vec![],
        });

        let node2 =  Rc::new(TreeNode {
            value: 2,
            children: vec![Rc::clone(&node5), Rc::clone(&node6)],
        });

        let node1 = Rc::new(TreeNode {
            value: 1,
            children: vec![Rc::clone(&node2), Rc::clone(&node3), Rc::clone(&node4)],
        });

        let tree = RcTree {
            root: node1,
        };

        println!("{}", tree);
    }

    #[test]
    fn exercise_07() {
        let node6 = RefCell::new(RefCellTreeNode {
            value: 6,
            children: vec![],
        });

        let node5 =  RefCell::new(RefCellTreeNode {
            value: 5,
            children: vec![],
        });

        let node4 =  RefCell::new(RefCellTreeNode {
            value: 4,
            children: vec![],
        });

        let node3 =  RefCell::new(RefCellTreeNode {
            value: 3,
            children: vec![],
        });

        let node2 =  RefCell::new(RefCellTreeNode {
            value: 2,
            children: vec![node5, node6],
        });

        let node1 = RefCell::new(RefCellTreeNode {
            value: 1,
            children: vec![node2, node3, node4],
        });

        let tree = RefCellTree {
            root: node1,
        };

        println!("{}", tree);
    }

    #[test]
    fn exercise_08() {
        let node6 = Rc::new(RefCell::new(RcRefCellTreeNode {
            value: 6,
            acc: 0,
            children: vec![],
        }));

        let node5 =  Rc::new(RefCell::new(RcRefCellTreeNode {
            value: 5,
            acc: 0,
            children: vec![],
        }));

        let node4 =  Rc::new(RefCell::new(RcRefCellTreeNode {
            value: 4,
            acc: 0,
            children: vec![],
        }));

        let node3 =  Rc::new(RefCell::new(RcRefCellTreeNode {
            value: 3,
            acc: 0,
            children: vec![],
        }));

        let node2 =  Rc::new(RefCell::new(RcRefCellTreeNode {
            value: 2,
            acc: 0,
            children: vec![node5, node6],
        }));

        let node1 = Rc::new(RefCell::new(RcRefCellTreeNode {
            value: 1,
            acc: 0,
            children: vec![node2, node3, node4],
        }));

        let tree = RcRefCellTree {
            root: node1,
        };

        Solution::execute_cost(&tree);

        println!("{}", tree);
    }

    #[test]
    fn exercise_09() {
        let node6 = Rc::new(RefCell::new(WeakTreeNode {
            value: 6,
            acc: 0,
            children: vec![],
            parent: Weak::new(),
            operation: Operation::NoOp,
        }));

        let node5 =  Rc::new(RefCell::new(WeakTreeNode {
            value: 5,
            acc: 0,
            children: vec![],
            parent: Weak::new(),
            operation: Operation::NoOp,
        }));

        let node4 =  Rc::new(RefCell::new(WeakTreeNode {
            value: 4,
            acc: 0,
            children: vec![],
            parent: Weak::new(),
            operation: Operation::NoOp,
        }));

        let node3 =  Rc::new(RefCell::new(WeakTreeNode {
            value: 3,
            acc: 0,
            children: vec![],
            parent: Weak::new(),
            operation: Operation::NoOp,
        }));

        let node2 =  Rc::new(RefCell::new(WeakTreeNode {
            value: 2,
            acc: 0,
            children: vec![Rc::clone(&node5), Rc::clone(&node6)],
            parent: Weak::new(),
            operation: Operation::Mult,
        }));

        node5.as_ref().borrow_mut().parent = Rc::downgrade(&node2);
        node6.as_ref().borrow_mut().parent = Rc::downgrade(&node2);

        let node1 = Rc::new(RefCell::new(WeakTreeNode {
            value: 1,
            acc: 0,
            children: vec![Rc::clone(&node2), Rc::clone(&node3), Rc::clone(&node4)],
            parent: Weak::new(),
            operation: Operation::Sum,
        }));

        node2.as_ref().borrow_mut().parent = Rc::downgrade(&node1);
        node3.as_ref().borrow_mut().parent = Rc::downgrade(&node1);
        node4.as_ref().borrow_mut().parent = Rc::downgrade(&node1);

        let tree = WeakTree {
            root: node1,
        };

        Solution::calculate(&tree);

        println!("{}", tree);
    }

    #[test]
    fn exercise_09_1() {
         // Create root node
        let root: Rc<RefCell<WeakTreeNode>> = WeakTreeNode::new(10, 0, Operation::Sum);

        // Create child nodes
        let child1 = WeakTreeNode::new(20, 0, Operation::Mult);
        let child2 = WeakTreeNode::new(30, 0, Operation::Mult);

        // Add children to root
        WeakTreeNode::add_child(&root, Rc::clone(&child1));
        WeakTreeNode::add_child(&root, Rc::clone(&child2));

        // Create tree structure
        let _tree = WeakTree { root: Rc::clone(&root) };

        // Adding grandchildren to child1
        let grandchild = WeakTreeNode::new(40, 0, Operation::Mult);
        WeakTreeNode::add_child(&child1, Rc::clone(&grandchild));

        // Debug output to check tree structure
        println!("Root: {:?}", root.borrow().value);
        println!("Root's first child: {:?}", root.borrow().children[0].borrow().value);
        println!("Root's second child: {:?}", root.borrow().children[1].borrow().value);
        println!("Child1's first child (grandchild): {:?}", child1.borrow().children[0].borrow().value);

        root.borrow_mut().children.clear();
        drop(child1);

        // Check parent references to ensure there's no memory leak
        if let Some(parent) = grandchild.borrow().parent.upgrade() {
            println!("Grandchild's parent value: {:?}", parent.borrow().value);
        } else {
            println!("Grandchild's parent has been deallocated");
        };
    }

    #[test]
    fn exercise_10() {
        {
            let my_pointer = FileSmartPointer::new("filename.txt");
            writeln!(&my_pointer.file, "hello world!").expect("could not write to file!");
        }

        // file is closed here.
        println!("The file has been closed as my_pointer went out of scope.");
    }
}