use crate::lexer::*;
use crate::object::*;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

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
    loop {
        let Some(token) = tokens.pop() else {
            return Err(ParseError {
                err: format!("Expected RParen or next list element, but ran out of tokens!"),
            });
        };
        match token {
            Token::Keyword(k) => list.push(Object::Keyword(k)),
            Token::If => list.push(Object::If),
            Token::BinaryOp(b) => list.push(Object::BinaryOp(b)),
            Token::Integer(n) => list.push(Object::Integer(n)),
            Token::Float(f) => list.push(Object::Float(f)),
            Token::String(s) => list.push(Object::String(s)),
            Token::Symbol(s) => list.push(Object::Symbol(s)),
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Object::List(Rc::new(list)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incomplete_list() {
        parse_list(&mut vec![Token::LParen, Token::Integer(5)])
            .expect_err("List is missing RParen");
    }

    #[test]
    fn test_add() {
        let list = parse("(+ 1 2)").unwrap();
        assert_eq!(
            list,
            Object::List(Rc::new(vec![
                Object::BinaryOp("+".to_string()),
                Object::Integer(1),
                Object::Integer(2),
            ]))
        );
    }

    #[test]
    fn test_area_of_a_circle() {
        let program = "(
                         (define r 10)
                         (define pi 314)
                         (* pi (* r r))
                       )";
        let list = parse(program).unwrap();
        assert_eq!(
            list,
            Object::List(Rc::new(vec![
                Object::List(Rc::new(vec![
                    Object::Keyword("define".to_string()),
                    Object::Symbol("r".to_string()),
                    Object::Integer(10),
                ])),
                Object::List(Rc::new(vec![
                    Object::Keyword("define".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::Integer(314),
                ])),
                Object::List(Rc::new(vec![
                    Object::BinaryOp("*".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::List(Rc::new(vec![
                        Object::BinaryOp("*".to_string()),
                        Object::Symbol("r".to_string()),
                        Object::Symbol("r".to_string()),
                    ])),
                ])),
            ]))
        );
    }
}
