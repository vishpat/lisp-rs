use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Symbol(String),
    Define,
    List(Vec<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(n) => write!(f, "{}", n),
            Object::Float(n) => write!(f, "{}", n),
            Object::Symbol(s) => write!(f, "{}", s),
            Object::Define => write!(f, "define"),
            Object::List(list) => {
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