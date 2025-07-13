pub mod error;

use crate::lox::{
    scanner::error::ScanningError,
    token::{Token, TokenType},
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<ScanningError>,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    fn scan_token(&mut self) {
        // let c = self.advance();

        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token = if self.advance_if_matching('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token)
            }
            '=' => {
                let token = if self.advance_if_matching('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token)
            }
            '<' => {
                let token = if self.advance_if_matching('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token)
            }
            '>' => {
                let token = if self.advance_if_matching('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token)
            }
            '/' => {
                if self.advance_if_matching('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            c => {
                if c.is_ascii_digit() {
                    self.number();
                } else {
                    self.errors.push(ScanningError {
                        _type: error::Type::UnexpectedCharacter,
                        line: self.line,
                    })
                }
            }
        }
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(TokenType::Number(
            self.source[self.start..self.current].parse().unwrap(),
        ));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(ScanningError {
                _type: error::Type::UnterminatedString,
                line: self.line,
            });
            return;
        }

        self.advance();

        self.add_token(TokenType::String(
            self.source[self.start + 1..self.current - 1].to_owned(),
        ));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn advance(&mut self) -> char {
        let next = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        next
    }

    fn advance_if_matching(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            _type: token_type,
            lexeme: self.source[self.start..self.current].to_string(),
            line: self.line,
        });
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<ScanningError>> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            _type: TokenType::Eof,
            lexeme: String::new(),
            line: self.line,
        });

        if self.errors.is_empty() {
            Ok(self.tokens.to_vec())
        } else {
            Err(self.errors.to_vec())
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
