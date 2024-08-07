# Arc<T>

`Arc<T>` is an atomically reference-counted smart pointer used for sharing data between threads. It is similar to Rc<T>, but it is thread-safe, making it suitable for concurrent programming.

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let a = Arc::new(5);

    let handles: Vec<_> = (0..10).map(|_| {
        let a_clone = Arc::clone(&a);
        thread::spawn(move || {
            println!("Value: {}", a_clone);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
```