#[cfg(test)]
mod tests{
    use decorator_macros::{logging, cache};

    #[logging]
    #[cache]
    fn my_service(input: &str) -> String {
        format!("Processed: {}", input)
    }

    #[test]
    fn decorator_example() {
        let input = "Hello, World!";
        println!("Result: {}", my_service(input)); // Logging + Cache miss
        println!("Result: {}", my_service(input)); // Logging + Cache hit
    }
}