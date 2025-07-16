pub mod error;
pub mod types;
pub mod scanner;
mod token;
mod parser;

use crate::lox::{
    error::{Error, Result},
    scanner::Scanner,
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
            .map_err(|errors| Error::Scanning(errors))?
        {
            println!("{token:#?}");
        }

        Ok(())
    }
}
