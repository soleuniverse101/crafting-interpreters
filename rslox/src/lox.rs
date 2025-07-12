pub mod error;
pub mod scanner;
mod token;

use crate::lox::{
    error::{Error, Result},
    scanner::{Scanner, error::ScanningError},
};

struct Context {
    had_error: bool,
}

impl Context {
    pub fn new() -> Context {
        Context { had_error: false }
    }
}

pub struct Lox {
    context: Context,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            context: Context::new(),
        }
    }

    pub fn run(&mut self, script: &str) -> Result<()> {
        println!("Running :\n{script}");
        let mut scanner = Scanner::new(script);

        for token in scanner
            .scan_tokens()
            .map_err(|ScanningError { lines }| Error::Scanning { lines })?
        {
            println!("{token:#?}");
        }

        Ok(())
    }
}
