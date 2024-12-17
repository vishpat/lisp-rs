use std::collections::HashSet;
use std::error::Error;
use std::str::Chars;
use std::{fmt, vec};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
    Float(f64),
    String(String),
    BinaryOp(String),
    Keyword(String),
}

#[derive(Debug)]
pub struct TokenError {
    err: String,
}

impl Error for TokenError {}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tokenization error: {}", self.err)
    }
}

struct Tokenizer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
    keywords: HashSet<&'a str>,
    binary_ops: HashSet<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();

        let keywords: HashSet<&str> = vec![
            "define", "list", "print", "lambda", "map", "filter", "reduce", "range", "car", "cdr",
            "length", "null?", "begin", "let", "if", "else", "cond",
        ]
        .into_iter()
        .collect::<HashSet<&str>>();

        let binary_ops: HashSet<char> = vec!['+', '-', '*', '/', '%', '<', '>', '=', '|', '&']
            .into_iter()
            .collect::<HashSet<char>>();

        Tokenizer {
            input: chars,
            current_char,
            keywords,
            binary_ops,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current_char = self.input.next();
        self.current_char
    }

    fn eat_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_symbol(&mut self) -> String {
        let mut symbol = String::new();
        while let Some(c) = self.current_char {
            if c.is_whitespace() || c == '(' || c == ')' || c == '\'' {
                break;
            }
            symbol.push(c);
            self.advance();
        }
        symbol
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while let Some(c) = self.current_char {
            if !c.is_numeric() && c != '.' {
                break;
            }
            number.push(c);
            self.advance();
        }
        number
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.advance(); // Skip the opening quote
        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance(); // Skip the closing quote
                break;
            }
            string.push(c);
            self.advance();
        }
        string
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.eat_whitespace();

        match self.current_char? {
            '(' => {
                self.advance();
                Some(Token::LParen)
            }
            ')' => {
                self.advance();
                Some(Token::RParen)
            }
            '"' => Some(Token::String(self.read_string())),
            c if c.is_numeric() => {
                let val = self.read_number();
                if val.contains('.') {
                    Some(Token::Float(val.parse().unwrap()))
                } else {
                    Some(Token::Integer(val.parse().unwrap()))
                }
            }
            c if c.is_alphabetic() || self.binary_ops.contains(&c) => {
                let sym = self.read_symbol();
                if self.keywords.contains(sym.as_str()) {
                    Some(Token::Keyword(sym))
                } else if self.binary_ops.contains(&sym.chars().next().unwrap()) {
                    Some(Token::BinaryOp(sym))
                } else {
                    Some(Token::Symbol(sym))
                }
            }
            _ => None,
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
    let mut tokenizer = Tokenizer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = tokenizer.next_token() {
        tokens.push(token);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let tokens = tokenize("(+ 1 2)").unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::BinaryOp("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_area_of_a_circle() {
        let program = "
            (
                (define r 10)
                (define pi 314)
                (* pi (* r r))
            )
        ";
        let tokens = tokenize(program).unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::LParen,
                Token::Keyword("define".to_string()),
                Token::Symbol("r".to_string()),
                Token::Integer(10),
                Token::RParen,
                Token::LParen,
                Token::Keyword("define".to_string()),
                Token::Symbol("pi".to_string()),
                Token::Integer(314),
                Token::RParen,
                Token::LParen,
                Token::BinaryOp("*".to_string()),
                Token::Symbol("pi".to_string()),
                Token::LParen,
                Token::BinaryOp("*".to_string()),
                Token::Symbol("r".to_string()),
                Token::Symbol("r".to_string()),
                Token::RParen,
                Token::RParen,
                Token::RParen
            ]
        );
    }
}
