use crate::lexer::*;
use crate::object::*;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    err: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

impl Error for ParseError {}

pub fn parse(program: &str) -> Result<Object, ParseError> {
    let token_result = tokenize(program);
    if token_result.is_err() {
        return Err(ParseError {
            err: format!("{}", token_result.err().unwrap()),
        });
    }

    let mut tokens = token_result.unwrap().into_iter().rev().collect::<Vec<_>>();
    let parsed_list = parse_list(&mut tokens)?;
    Ok(parsed_list)
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {
    let token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err(ParseError {
            err: format!("Expected LParen, found {:?}", token),
        });
    }

    let mut list: Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token == None {
            return Err(ParseError {
                err: format!("Did not find enough tokens"),
            });
        }
        let t = token.unwrap();
        match t {
            Token::Integer(n) => list.push(Object::Integer(n)),
            Token::Symbol(s) => list.push(Object::Symbol(s)),
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Object::List(list));
            }
        }
    }

    Ok(Object::List(list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let list = parse("(+ 1 2)").unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::Symbol("+".to_string()),
                Object::Integer(1),
                Object::Integer(2),
            ])
        );
    }

    #[test]
    fn test_area_of_a_circle() {
        let list = parse("(begin (define r 10)(define pi 3.14)(* pi (* r r)))").unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::Symbol("begin".to_string()),
                Object::List(vec![
                    Object::Symbol("define".to_string()),
                    Object::Symbol("r".to_string()),
                    Object::Integer(10),
                ]),
                Object::List(vec![
                    Object::Symbol("define".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::Float(3.14),
                ]),
                Object::List(vec![
                    Object::Symbol("*".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::List(vec![
                        Object::Symbol("*".to_string()),
                        Object::Symbol("r".to_string()),
                        Object::Symbol("r".to_string()),
                    ]),
                ]),
            ])
        );
    }
}
