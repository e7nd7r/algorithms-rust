use std::{cell::RefCell, collections::HashMap};

pub trait Service {
    fn execute(&self, input: &str) -> String;
}

pub struct BasicService;

impl Service for BasicService {
    fn execute(&self, input: &str) -> String {
        format!("Processed: {}", input)
    }
}

pub struct LoggingDecorator<T: Service> {
    inner: T,
}

impl<T: Service> LoggingDecorator<T> {
    fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Service> Service for LoggingDecorator<T> {
    fn execute(&self, input: &str) -> String {
        println!("Logging: Executing with input: {}", input);
        let result = self.inner.execute(input);
        println!("Logging: Execution result: {}", result);
        result
    }
}


pub struct CachingDecorator<T: Service> {
    inner: T,
    cache: RefCell<HashMap<String, String>>,
}

impl<T: Service> CachingDecorator<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            cache: RefCell::new(HashMap::new()),
        }
    }
}

impl<T: Service> Service for CachingDecorator<T> {
    fn execute(&self, input: &str) -> String {
        if let Some(cached_result) = self.cache.borrow().get(input) {
            println!("Cache hit for input: {}", input);
            return cached_result.clone();
        }

        println!("Cache miss for input: {}", input);
        let result = self.inner.execute(input);
        self.cache.borrow_mut().insert(input.to_string(), result.clone());
        result
    }
}

#[cfg(test)]
mod test {
    use crate::learning_series::serie2_decorator::*;

    #[test]
    fn decorator_example() {
        // Create a basic service
        let basic_service = BasicService;

        // Wrap it with the logging decorator
        let logged_service = LoggingDecorator::new(basic_service);

        // Wrap the logging-decorated service with the caching decorator
        let cached_logged_service = CachingDecorator::new(logged_service);

        // Use the composed service
        let input = "Hello, World!";
        println!("Result: {}", cached_logged_service.execute(input)); // Logging + Cache miss
        println!("Result: {}", cached_logged_service.execute(input)); // Logging + Cache hit
    }
}
