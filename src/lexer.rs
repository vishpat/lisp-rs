use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    Symbol(String),
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Symbol(s) => write!(f, "{}", s),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
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

pub fn tokenize(program: &str) -> Result<Vec<Token>, TokenError>
{
    let program2 = program.replace("(", " ( ").replace(")", " ) "); 
    let words = program2.split_whitespace();
    let mut tokens: Vec<Token> = Vec::new();
    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                let mut chars = word.chars();
                let first_char = chars.next().unwrap();
                if first_char.is_digit(10) {
                    let num = word.parse::<i64>();
                    if num.is_err() {
                        return Err(TokenError::new(first_char));
                    }
                    tokens.push(Token::Number(num.unwrap()));
                } else {
                    tokens.push(Token::Symbol(word.to_string()));
                }
            }
        }
    }
    Ok(tokens)
}

