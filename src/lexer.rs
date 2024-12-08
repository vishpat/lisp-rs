use std::collections::HashSet;
use std::error::Error;
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
    If,
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

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
    let keywords: HashSet<&str> = vec![
        "define", "list", "print", "lambda", "map", "filter", "reduce", "range", "car", "cdr",
        "length", "null?", "begin", "let", "if",
    ]
    .into_iter()
    .collect::<HashSet<&str>>();

    let binary_ops: HashSet<&str> = vec!["+", "-", "*", "/", "%", "<", ">", "=", "!="]
        .into_iter()
        .collect::<HashSet<&str>>();

    let mut tokens = Vec::new();
    let mut chars = input.chars().collect::<Vec<char>>();

    if chars.is_empty() {
        return Ok(tokens);
    }

    while !chars.is_empty() {
        let mut ch = chars.remove(0);
        match ch {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '"' => {
                let mut word = String::new();
                while !chars.is_empty() && chars[0] != '"' {
                    word.push(chars.remove(0));
                }

                if !chars.is_empty() && chars[0] == '"' {
                    chars.remove(0);
                } else {
                    return Err(TokenError {
                        err: format!("Unterminated string: {}", word),
                    });
                }

                tokens.push(Token::String(word));
            }
            _ => {
                let mut word = String::new();
                while !chars.is_empty() && !ch.is_whitespace() && ch != '(' && ch != ')' {
                    word.push(ch);
                    let peek = chars[0];
                    if peek == '(' || peek == ')' {
                        break;
                    }

                    ch = chars.remove(0);
                }

                if !word.is_empty() {
                    tokens.push(if let Ok(i) = word.parse::<i64>() {
                        Token::Integer(i)
                    } else if let Ok(f) = word.parse::<f64>() {
                        Token::Float(f)
                    } else if word == "if" {
                        Token::If
                    } else if binary_ops.contains(word.as_str()) || word == "or" || word == "and" {
                        Token::BinaryOp(word)
                    } else if keywords.contains(word.as_str()) {
                        Token::Keyword(word)
                    } else {
                        Token::Symbol(word)
                    });
                }
            }
        }
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
