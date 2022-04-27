use std::fmt;
use crate::object::Object;

pub enum Token {
    Number(i64),
    Symbol(String),
    LParen,
    RParen,
}

pub struct TokenError {
    ch: char,
}

impl TokenError {
    pub fn new(ch: char) -> TokenError {
        TokenError { ch }
    }
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected character: {}", self.ch)
    }
}

fn tokenize(program: &str) -> Result<Vec<Object>, TokenError>
{
    program.split_whitespace()
        .map(|token| token.parse().unwrap())
        .collect()
}

