use std::env;

use rslox::{lox::error::Error, run_file, run_prompt};
use sysexits::ExitCode;

fn main() -> ExitCode {
    let args: Vec<_> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        ExitCode::Usage
    } else if args.len() == 2 {
        let Err(err) = run_file(&args[1]) else {
            return ExitCode::Ok;
        };

        match err {
            Error::Scanning { lines: _ } => ExitCode::DataErr,
        }
    } else {
        run_prompt();
        ExitCode::Ok
    }
}
