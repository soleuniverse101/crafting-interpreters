use crate::lox::{Lox, scanner::error::ScanningError};

pub enum Error {
    Scanning(Vec<ScanningError>),
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

pub fn print_error(error: &Error) {
    match error {
        Error::Scanning(errors) => {
            for error in errors {
                eprintln!("{} error encountered on line {}", error._type, error.line)
            }
        }
    }
}
