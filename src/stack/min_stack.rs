/**
 * 
 */
struct MinStack {
    inner: Vec<(i32, i32)>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            inner: Vec::new()
        }
    }
    
    fn push(&mut self, val: i32) {
        let last_value = self.inner.last();

        let new_value = match last_value {
            None => (val, val),
            Some((_, min)) if val > *min => (val, *min),
            _ => (val, val)
        };

        self.inner.push(new_value);
    }
    
    fn pop(&mut self) {
        if self.inner.is_empty() {
            return ();
        }

        self.inner.pop();
    }
    
    fn top(&self) -> i32 {
        self.inner.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.inner.last().unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn test_stack_ops() {
        let mut min_stack = MinStack::new();

        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3); // return -3
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);    // return 0
        assert_eq!(min_stack.get_min(), -2); // return -2
    }

    #[test]
    fn test_stack_repeated_values() {
        let mut min_stack = MinStack::new();

        min_stack.push(-2);
        min_stack.push(-2);
        min_stack.push(-2);
        min_stack.push(-3);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3); // return -3
        assert_eq!(min_stack.get_min(), -3); // return -3
        min_stack.pop();
        min_stack.pop();
        assert_eq!(min_stack.get_min(), -2); // return -3
    }

    #[test]
    fn test_stack_repeated_values2() {
        let mut min_stack = MinStack::new();

        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-1);
        assert_eq!(min_stack.get_min(), -2); // return -3
        assert_eq!(min_stack.top(), -1); // return -3
        min_stack.pop();
        assert_eq!(min_stack.get_min(), -2); // return -3
    }
}
