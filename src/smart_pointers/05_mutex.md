# Mutex<T>

`Mutex<T>` is used to ensure mutual exclusion, protecting shared data across threads by allowing only one thread to access the data at a time. It provides interior mutability similar to RefCell<T> but is safe for concurrent use.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..10).map(|_| {
        let data_clone = Arc::clone(&data);
        thread::spawn(move || {
            let mut num = data_clone.lock().unwrap();
            *num += 1;
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *data.lock().unwrap());
}
```