use crate::env::*;
use crate::object::*;

fn eval_infix(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for infix operator"));
    }
    let operator = list[0].clone();
    let left = eval(&list[1].clone(), env)?;
    let right = eval(&list[2].clone(), env)?;
    let left_val = match left {
        Object::Integer(n) => n,
        _ => return Err(format!("Left operand must be an integer {:?}", left)),
    };
    let right_val = match right {
        Object::Integer(n) => n,
        _ => return Err(format!("Right operand must be an integer {:?}", right)),
    };
    match operator {
        Object::Symbol(s) => match s.as_str() {
            "+" => Ok(Object::Integer(left_val + right_val)),
            "-" => Ok(Object::Integer(left_val - right_val)),
            "*" => Ok(Object::Integer(left_val * right_val)),
            "/" => Ok(Object::Integer(left_val / right_val)),
            _ => Err(format!("Invalid infix operator: {}", s)),
        },
        _ => Err(format!("Operator must be a symbol")),
    }
}

pub fn eval(obj: &Object, env: &mut Env) -> Result<Object, String> {
    match obj {
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Symbol(s) => {
            let val = env.get(s);
            if val.is_none() {
                return Err(format!("Unbound symbol: {}", s));
            }
            return Ok(val.unwrap().clone());
        }
        Object::List(list) => {
            let head = &list[0];
            match head {
                Object::Symbol(s) => match s.as_str() {
                    "+" | "-" | "*" | "/" => {
                        return eval_infix(&list, env);
                    }
                    _ => {
                        return Err(format!("Unknown symbol: {}", s));
                    }
                },
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
