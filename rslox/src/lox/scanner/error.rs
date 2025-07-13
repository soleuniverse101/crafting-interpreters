use std::fmt;

#[derive(Debug, Clone)]
pub enum Type {
    UnexpectedCharacter,
    UnterminatedString,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UnexpectedCharacter => "Unexpected character",
                Self::UnterminatedString => "Unterminated string",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct ScanningError {
    pub _type: Type,
    pub line: usize,
}
