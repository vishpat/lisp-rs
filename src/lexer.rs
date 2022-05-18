use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Float(f64),
    String(String),
    Binary_OP(String),
    Keyword(String),
    Symbol(String),
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::Binary_OP(s) => write!(f, "{}", s),
            Token::String(s) => write!(f, "{}", s),
            Token::Symbol(s) => write!(f, "{}", s),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Keyword(s) => write!(f, "{}", s),
        }
    }
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
    let mut tokens = Vec::new();
    let mut chars = input.chars().collect::<Vec<char>>();

    if chars.is_empty() {
        return Ok(tokens);
    }

    while chars.len() > 0 {
        let mut ch = chars.remove(0);
        match ch {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '"' => {
                let mut word = String::new();
                while chars.len() > 0 && chars[0] != '"' {
                    word.push(chars.remove(0));
                }

                if chars.len() > 0 && chars[0] == '"' {
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
                while chars.len() > 0 && !ch.is_whitespace() && ch != '(' && ch != ')' {
                    word.push(ch);
                    let peek = chars[0];
                    if peek == '(' || peek == ')' {
                        break;
                    }

                    ch = chars.remove(0);
                }

                if word.is_empty() {
                    continue;
                }

                let i = word.parse::<i64>();
                if i.is_ok() {
                    tokens.push(Token::Integer(i.unwrap()));
                    continue;
                }

                let f = word.parse::<f64>();
                if f.is_ok() {
                    tokens.push(Token::Float(f.unwrap()));
                    continue;
                }

                let token = match word.as_str() {
                    "define" | "if" | "list" | "print" | "lambda" | "map" | "filter" | "reduce" => {
                        Token::Keyword(word)
                    }
                    "+" | "-" | "*" | "/" | "%" | "<" | ">" | "==" | "!=" | "&" | "|" => {
                        Token::Binary_OP(word)
                    }
                    _ => Token::Symbol(word),
                };

                tokens.push(token);
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
                Token::Symbol("+".to_string()),
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
                Token::Symbol("*".to_string()),
                Token::Symbol("pi".to_string()),
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("r".to_string()),
                Token::Symbol("r".to_string()),
                Token::RParen,
                Token::RParen,
                Token::RParen
            ]
        );
    }
}
