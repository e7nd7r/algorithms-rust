use std::fs::{ OpenOptions, File };
use std::io::{ self, Write };
use std::sync::{ Arc, Mutex, OnceLock };

pub struct Logger {
    file: Mutex<File>,
}

impl Logger {
    fn new() -> io::Result<Logger> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("app.log")?;

        Ok(Logger{
            file: Mutex::new(file),
        })
    }

    pub fn instance() -> Arc<Logger> {
        static INSTANCE : OnceLock<Arc<Logger>> = OnceLock::new();

        INSTANCE.get_or_init(|| Arc::new(Logger::new().unwrap())).clone()
    }

    pub fn log(&self, message: &str) {
        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", message).expect("file to write to log file");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn singleton_test() {
        let logger = Logger::instance();
        let logger2 = Logger::instance();

        assert!(ptr::eq(logger.as_ref(), logger2.as_ref()));
    }
}