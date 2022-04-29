use crate::env::*;
use crate::object::*;
use crate::parser::*;

fn eval_infix(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for infix operator"));
    }
    let operator = list[0].clone();
    let left = eval_list(&list[1].clone(), env)?;
    let right = eval_list(&list[2].clone(), env)?;
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

fn eval_list(obj: &Object, env: &mut Env) -> Result<Object, String> {
    match obj {
        Object::Void => Ok(Object::Void),
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
            println!("Evaluating list with head {:?}", head);
            match head {
                Object::Symbol(s) => match s.as_str() {
                    "+" | "-" | "*" | "/" => {
                        return eval_infix(&list, env);
                    }
                    "define" => {
                        let sym = match &list[1] {
                            Object::Symbol(s) => s.clone(),
                            _ => return Err(format!("Invalid define")),
                        };
                        let val = eval_list(&list[2], env)?;
                        env.set(&sym, val);
                        return Ok(Object::Void);
                    }
                    _ => {
                        return Err(format!("Unknown symbol: {}", s));
                    }
                },
                _ => {
                    let mut new_list = Vec::new();
                    for obj in list {
                        let result = eval_list(obj, env)?;
                        match result {
                            Object::Void => {}
                            _ => new_list.push(result),
                        }
                    }
                    Ok(Object::List(new_list))
                }
            }
        }
    }
}

pub fn eval(program: &str, env: &mut Env) -> Result<Object, String> {
    let parsed_list = parse(program);
    if parsed_list.is_err() {
        return Err(format!("{}", parsed_list.err().unwrap()));
    }
    eval_list(&parsed_list.unwrap(), env)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let result = eval("(+ 1 2)", &mut Env::new()).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn test_area_of_a_circle() {
        let result = eval(
            "((define r 10) (define pi 314) (* pi (* r r)))",
            &mut Env::new(),
        )
        .unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((314 * 10 * 10) as i64)])
        );
    }
}
