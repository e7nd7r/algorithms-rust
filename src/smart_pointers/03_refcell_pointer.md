# RefCell<T>
RefCell<T> is used for interior mutability, allowing you to mutate data even when there are immutable references. It enforces borrow rules at runtime instead of compile time, allowing more flexibility but potentially causing runtime errors if not used carefully.

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    // Borrow as immutable
    {
        let value = data.borrow();
        println!("Value: {}", value);
    }

    // Borrow as mutable
    {
        let mut value = data.borrow_mut();
        *value += 1;
    }

    println!("Updated Value: {}", data.borrow());
}
```
