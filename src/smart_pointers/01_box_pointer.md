# Box<T>

`Box<T>`  allows you to store data on the heap instead of the stack. It has a known size and implements the Deref and Drop traits, allowing for smart pointer behavior and automatic deallocation.

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```