use crate::env::*;
use crate::object::*;

pub fn eval(obj: &Object, env: &mut Env) -> Result<Object, String> {
    match obj {
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Float(n) => Ok(Object::Float(*n)),
        Object::Symbol(s) => {
            let val = env.get(s);
            if val.is_none() {
                return Err(format!("Unbound symbol: {}", s));
            }
            return Ok(val.unwrap().clone());
        }
        Object::List(list) => {
            let head = list[0];
            match head {
                Object::Symbol(s) => {
                    match s.as_str() {
                        "+" => {},
                        "-" => {},
                        "*" => {},
                        "/" => {},
                        "define" => {},
                        _ => {
                            return Err(format!("Unknown symbol: {}", s));
                        }
                    } 
                }
                _ => {
                    let mut new_list = Vec::new();
                    for obj in list {
                        new_list.push(eval(obj, env)?);
                    }
                    Ok(Object::List(new_list))
                }
            }
        }
    }
}
