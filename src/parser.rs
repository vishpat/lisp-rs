use crate::lexer::*;
use crate::object::*;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct ParseError {
    token: Token,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected token: {}", self.token)
    }
}

impl Error for ParseError {}
    


pub fn parse_list(tokens: &mut Vec<Token>) -> Result<Vec<Object>, ParseError> {
    println!("parse_list: {:?}", tokens);

    let token = tokens.pop();
    if token != Some(Token::LParen) {
        println!("Did not find LParen {:?}", token);
        return Err(ParseError {
            token: token.unwrap(),
        });
    }

    let mut list: Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token == None {
            println!("Did not find token {:?}", token);
            return Err(ParseError {
                token: Token::Invalid,
            });
        }
        println!("token: {:?}", token);
        let t = token.unwrap();
        match t {
            Token::Integer(n) => list.push(Object::Integer(n)),
            Token::Float(n) => list.push(Object::Float(n)),
            Token::Symbol(s) => list.push(Object::Symbol(s)),
            Token::Define => list.push(Object::Define),
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list( tokens)?;
                list.push(Object::List(sub_list));
            }
            Token::RParen => {
                return Ok(list);
            }
            _ => {
                println!("Unexpected token {:?}", t);
                return Err(ParseError {
                    token: Token::Invalid,
                })
            }
        }
    }

    Ok(list)
}
