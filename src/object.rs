use crate::env::*;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Void,
    Keyword(String),
    BinaryOp(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Symbol(String),
    ListData(Vec<Object>),
    Lambda(Vec<String>, Rc<Vec<Object>>, Rc<RefCell<Env>>),
    List(Rc<Vec<Object>>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Void => write!(f, "Void"),
            Object::Keyword(s) => write!(f, "{}", s),
            Object::BinaryOp(s) => write!(f, "{}", s),
            Object::Integer(n) => write!(f, "{}", n),
            Object::Float(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Symbol(s) => write!(f, "{}", s),
            Object::String(s) => write!(f, "{}", s),
            Object::Lambda(params, body, _env) => {
                write!(f, "Lambda(")?;
                for param in params {
                    write!(f, "{} ", param)?;
                }
                write!(f, ")")?;
                for expr in (*body).iter() {
                    write!(f, " {}", expr)?;
                }
                Ok(())
            }
            Object::List(list) => {
                write!(f, "(")?;
                for (i, obj) in (*list).iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", obj)?;
                }
                write!(f, ")")
            }
            Object::ListData(list) => {
                write!(f, "(")?;
                for (i, obj) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", obj)?;
                }
                write!(f, ")")
            }
        }
    }
}
