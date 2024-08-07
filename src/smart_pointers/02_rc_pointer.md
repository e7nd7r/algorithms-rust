# Rc<T>

Rc<T> is a reference-counted smart pointer used when you want multiple parts of your program to read the same data. It keeps track of the number of references to the data, allowing for shared ownership.

Note: Not safe for concurrent access. Use `Arc<T>` instead.

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    println!("a = {}, b = {}", a, b);
    println!("Reference count: {}", Rc::strong_count(&a));
}
```