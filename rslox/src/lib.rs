use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use crate::lox::error::{Error, Result};

pub mod lox;

pub fn run_file(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {} : {}", display, why),
        Ok(file) => file,
    };

    let mut script = String::new();
    match file.read_to_string(&mut script) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    let mut lox = lox::Lox::new();

    lox.run(&script)
}

pub fn run_prompt() {
    let mut lox = lox::Lox::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => (),
            Err(_) => break,
        }
        if let Err(err) = lox.run(line.trim()) {
            match err {
                Error::Scanning { lines } => match lines.len() {
                    1 => eprintln!("Scanning error encountered on line {}", lines[1]),
                    _ => {
                        eprint!("Scanning errors encountered on lines ");
                        for line in &lines[..lines.len() - 1] {
                            eprint!("{line}, ");
                        }
                        eprintln!("{}", lines.last().unwrap());
                    }
                },
            }
        }
    }
}
