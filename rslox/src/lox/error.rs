use crate::lox::Lox;

pub enum Error {
    Scanning { lines: Vec<usize> },
}

pub type Result<T> = std::result::Result<T, Error>;

impl Lox {
    fn report(&mut self, line: i32, location: &str, message: &str) {
        self.context.had_error = true;
        println!("[line {line}] Error {location}: {message}");
    }

    pub fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }
}
