use crate::env::*;
use crate::object::*;
use crate::parser::*;
use std::cell::RefCell;
use std::rc::Rc;

fn eval_infix(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
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

fn eval_list(obj: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    match obj {
        Object::Void => Ok(Object::Void),
        Object::Lambda(_params, _body) => Ok(Object::Void),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Symbol(s) => {
            let val = env.borrow_mut().get(s);
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
                        env.borrow_mut().set(&sym, val);
                        return Ok(Object::Void);
                    }
                    "lambda" => {
                        let params = match &list[1] {
                            Object::List(list) => {
                                let mut params = Vec::new();
                                for param in list {
                                    match param {
                                        Object::Symbol(s) => params.push(s.clone()),
                                        _ => return Err(format!("Invalid lambda parameter")),
                                    }
                                }
                                params
                            }
                            _ => return Err(format!("Invalid lambda")),
                        };

                        let body = match &list[2] {
                            Object::List(list) => list.clone(),
                            _ => return Err(format!("Invalid lambda")),
                        };
                        return Ok(Object::Lambda(params, body));
                    }
                    _ => {
                        let lamdba = env.borrow_mut().get(s);
                        if lamdba.is_none() {
                            return Err(format!("Unbound symbol: {}", s));
                        }

                        let func = lamdba.unwrap();
                        println!("Found lamdba for {} {:?}", s, func);
                        match func {
                            Object::Lambda(params, body) => {
                                let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
                                for (i, param) in params.iter().enumerate() {
                                    let val = eval_list(&list[i + 1], env)?;
                                    new_env.borrow_mut().set(param, val);
                                }
                                let new_body = body.clone();
                                return eval_list(&Object::List(new_body), &mut new_env);
                            }
                            _ => return Err(format!("Not a lambda: {}", s)),
                        }
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

pub fn eval(program: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
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
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(+ 1 2)", &mut env).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn test_area_of_a_circle() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("((define r 10) (define pi 314) (* pi (* r r)))", &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((314 * 10 * 10) as i64)])
        );
    }

    #[test]
    fn test_sqr_function() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("((define sqr (lambda (r) (* r r))) (sqr 10))", &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((10 * 10) as i64)])
        );
    }

    #[test]
    fn test_circle_area_function() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (
                (define pi 314)
                (define r 10)
                (define sqr (lambda (r) (* r r)))
                (define area (lambda (r) (* pi (sqr r))))
                (area r)
            )
        ";
        
        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((314 * 10 * 10) as i64)])
        );
    }
}
