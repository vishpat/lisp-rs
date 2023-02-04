use crate::env::*;
use crate::object::*;
use crate::parser::*;
use std::cmp::Ordering;

fn print_list(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    let mut new_list = Vec::new();

    for obj in list[1..].iter() {
        new_list.push(eval_obj(obj, env)?);
    }
    for obj in new_list.iter() {
        print!("{} ", obj);
    }
    println!("");
    Ok(Object::Void)
}

fn eval_binary_op(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for infix operator"));
    }
    let operator = list[0].clone();
    let left = &eval_obj(&list[1].clone(), env)?;
    let right = &eval_obj(&list[2].clone(), env)?;
    match operator {
        Object::Symbol(s) => match s.as_str() {
            "+" => match (left, right) {
                (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l + r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l + r)),
                (Object::Integer(l), Object::Float(r)) => Ok(Object::Float(*l as f64 + r)),
                (Object::Float(l), Object::Integer(r)) => Ok(Object::Float(l + *r as f64)),
                (Object::String(l), Object::String(r)) => Ok(Object::String(l.to_owned() + &*r)),
                _ => Err(format!("Invalid types for + operator {} {}", left, right)),
            },
            "-" => match (left, right) {
                (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l - r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l - r)),
                (Object::Integer(l), Object::Float(r)) => Ok(Object::Float(*l as f64 - r)),
                (Object::Float(l), Object::Integer(r)) => Ok(Object::Float(l - *r as f64)),
                _ => Err(format!("Invalid types for - operator {} {}", left, right)),
            },
            "*" => match (left, right) {
                (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l * r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l * r)),
                (Object::Integer(l), Object::Float(r)) => Ok(Object::Float(*l as f64 * r)),
                (Object::Float(l), Object::Integer(r)) => Ok(Object::Float(l * (*r) as f64)),
                _ => Err(format!("Invalid types for * operator {} {}", left, right)),
            },
            "/" => match (left, right) {
                (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l / r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l / r)),
                (Object::Integer(l), Object::Float(r)) => Ok(Object::Float(*l as f64 / r)),
                (Object::Float(l), Object::Integer(r)) => Ok(Object::Float(l / (*r) as f64)),
                _ => Err(format!("Invalid types for / operator {} {}", left, right)),
            },
            "%" => match (left, right) {
                (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l % r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l % r)),
                (Object::Integer(l), Object::Float(r)) => Ok(Object::Float(*l as f64 % r)),
                (Object::Float(l), Object::Integer(r)) => Ok(Object::Float(l % (*r) as f64)),
                _ => Err(format!("Invalid types for % operator {} {}", left, right)),
            },
            "<" => match (left, right) {
                (Object::Integer(l), Object::Integer(r)) => Ok(Object::Bool(l < r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Bool(l < r)),
                (Object::Integer(l), Object::Float(r)) => Ok(Object::Bool((*l as f64) < *r)),
                (Object::Float(l), Object::Integer(r)) => Ok(Object::Bool(l < &(*r as f64))),
                (Object::String(l), Object::String(r)) => {
                    Ok(Object::Bool(l.cmp(&r) == Ordering::Less))
                }
                _ => Err(format!("Invalid types for < operator {} {}", left, right)),
            },
            ">" => match (left, right) {
                (Object::Integer(l), Object::Integer(r)) => Ok(Object::Bool(l > r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Bool(l > r)),
                (Object::Integer(l), Object::Float(r)) => Ok(Object::Bool(*l as f64 > *r)),
                (Object::Float(l), Object::Integer(r)) => Ok(Object::Bool(l > &(*r as f64))),
                (Object::String(l), Object::String(r)) => {
                    Ok(Object::Bool(l.cmp(&r) == Ordering::Greater))
                }
                _ => Err(format!("Invalid types for > operator {} {}", left, right)),
            },
            "=" => match (left, right) {
                (Object::Integer(l), Object::Integer(r)) => Ok(Object::Bool(l == r)),
                (Object::String(l), Object::String(r)) => Ok(Object::Bool(l == r)),
                _ => Err(format!("Invalid types for == operator {} {}", left, right)),
            },
            "!=" => match (left, right) {
                (Object::Integer(l), Object::Integer(r)) => Ok(Object::Bool(l != r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Bool(l != r)),
                (Object::Integer(l), Object::Float(r)) => Ok(Object::Bool(*l as f64 != *r)),
                (Object::Float(l), Object::Integer(r)) => Ok(Object::Bool(*l != (*r) as f64)),
                (Object::String(l), Object::String(r)) => {
                    Ok(Object::Bool(l.cmp(&r) != Ordering::Equal))
                }
                _ => Err(format!("Invalid types for != operator {} {}", left, right)),
            },
            "&" => match (left, right) {
                (Object::Bool(l), Object::Bool(r)) => Ok(Object::Bool(*l && *r)),
                _ => Err(format!("Invalid types for & operator {} {}", left, right)),
            },
            "|" => match (left, right) {
                (Object::Bool(l), Object::Bool(r)) => Ok(Object::Bool(*l || *r)),
                _ => Err(format!("Invalid types for | operator {} {}", left, right)),
            },
            _ => Err(format!("Invalid infix operator: {}", s)),
        },
        _ => Err(format!("Operator must be a symbol")),
    }
}

fn eval_define(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for define"));
    }

    let sym = match &list[1] {
        Object::Symbol(s) => s.clone(),
        _ => return Err(format!("Invalid define")),
    };
    let val = eval_obj(&list[2], env)?;
    env.set(&sym, val);
    Ok(Object::Void)
}

fn eval_list_data(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    let mut new_list = Vec::new();

    for obj in list[1..].iter() {
        new_list.push(eval_obj(obj, env)?);
    }
    Ok(Object::ListData(new_list))
}

fn eval_if(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() != 4 {
        return Err(format!("Invalid number of arguments for if statement"));
    }

    let cond_obj = eval_obj(&list[1], env)?;
    let cond = match cond_obj {
        Object::Bool(b) => b,
        _ => return Err(format!("Condition must be a boolean")),
    };

    if cond == true {
        return eval_obj(&list[2], env);
    } else {
        return eval_obj(&list[3], env);
    }
}

fn eval_function_definition(list: &Vec<Object>) -> Result<Object, String> {
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
    Ok(Object::Lambda(params, body))
}

fn eval_function_call(s: &str, list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    let lamdba = env.get(s);
    if lamdba.is_none() {
        return Err(format!("Unbound function: {}", s));
    }

    let func = lamdba.unwrap();
    match func {
        Object::Lambda(params, body) => {
            let mut new_env = Box::new(Env::extend(env));
            for (i, param) in params.iter().enumerate() {
                let val = eval_obj(&list[i + 1], &mut *new_env)?;
                new_env.set(param, val);
            }
            return eval_obj(&Object::List(body), &mut *new_env);
        }
        _ => return Err(format!("Not a lambda: {}", s)),
    }
}

fn eval_symbol(s: &str, env: &mut Env) -> Result<Object, String> {
    let val = match s {
        "#t" => return Ok(Object::Bool(true)),
        "#f" => return Ok(Object::Bool(false)),
        "#nil" => return Ok(Object::Void),
        _ => env.get(s),
    };

    if val.is_none() {
        return Err(format!("Unbound symbol: {}", s));
    }

    Ok(val.unwrap().clone())
}

fn eval_map(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for map {:?}", list));
    }

    let lambda = eval_obj(&list[1], env)?;
    let arg_list = eval_obj(&list[2], env)?;

    let (params, body) = match lambda {
        Object::Lambda(p, b) => {
            if p.len() != 1 {
                return Err(format!(
                    "Invalid number of parameters for map lambda function {:?}",
                    p
                ));
            }
            (p, b)
        }
        _ => return Err(format!("Not a lambda while evaluating map: {}", lambda)),
    };

    let args = match arg_list {
        Object::ListData(list) => list,
        _ => return Err(format!("Invalid map arguments: {:?}", list)),
    };

    let func_param = &params[0];
    let mut result_list = Vec::new();
    for arg in args.iter() {
        let val = eval_obj(&arg, env)?;
        let mut new_env = Box::new(Env::extend(env));
        new_env.set(&func_param, val);
        let new_body = body.clone();
        let result = eval_obj(&Object::List(new_body), &mut *new_env)?;
        result_list.push(result);
    }
    Ok(Object::ListData(result_list))
}

fn eval_filter(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for filter {:?}", list));
    }

    let lambda = eval_obj(&list[1], env)?;
    let arg_list = eval_obj(&list[2], env)?;

    let (params, body) = match lambda {
        Object::Lambda(p, b) => {
            if p.len() != 1 {
                return Err(format!(
                    "Invalid number of parameters for map function {:?}",
                    p
                ));
            }
            (p, b)
        }
        _ => return Err(format!("Not a lambda while evaluating map: {}", lambda)),
    };

    let args = match arg_list {
        Object::ListData(list) => list,
        _ => return Err(format!("Invalid map arguments: {:?}", list)),
    };

    let func_param = &params[0];
    let mut result_list = Vec::new();
    for arg in args.iter() {
        let val = eval_obj(&arg, env)?;
        let mut new_env = Box::new(Env::extend(env));
        new_env.set(&func_param, val.clone());
        let new_body = body.clone();
        let result_obj = eval_obj(&Object::List(new_body), &mut *new_env)?;
        let result = match result_obj {
            Object::Bool(b) => b,
            _ => return Err(format!("Invalid filter result: {}", result_obj)),
        };
        if result {
            result_list.push(val);
        }
    }
    Ok(Object::ListData(result_list))
}

fn eval_reduce(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for reduce {:?}", list));
    }

    let lambda = eval_obj(&list[1], env)?;
    let arg_list = eval_obj(&list[2], env)?;

    let (params, body) = match lambda {
        Object::Lambda(p, b) => {
            if p.len() != 2 {
                return Err(format!(
                    "Invalid number of parameters for reduce function {:?}",
                    p
                ));
            }
            (p, b)
        }
        _ => return Err(format!("Not a lambda while evaluating map: {}", lambda)),
    };

    let args = match arg_list {
        Object::ListData(list) => list,
        _ => return Err(format!("Invalid map arguments: {:?}", list)),
    };

    if args.len() < 2 {
        return Err(format!(
            "Invalid number of arguments for reduce: {:?}",
            args
        ));
    }

    let reduce_param1 = &params[0];
    let reduce_param2 = &params[1];
    let mut accumulator = eval_obj(&args[0], env)?;

    for arg in args[1..].iter() {
        let mut new_env = Box::new(Env::extend(env));
        new_env.set(&reduce_param1, accumulator.clone());

        let val = eval_obj(&arg, &mut new_env)?;
        new_env.set(&reduce_param2, val.clone());

        let new_body = body.clone();
        accumulator = eval_obj(&Object::List(new_body), &mut *new_env)?;
    }
    Ok(accumulator)
}
fn eval_list(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    let head = &list[0];
    match head {
        Object::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "%" | "<" | ">" | "=" | "!=" | "&" | "|" => {
                return eval_binary_op(&list, env);
            }
            "define" => eval_define(&list, env),
            "if" => eval_if(&list, env),
            "list" => eval_list_data(&list, env),
            "print" => print_list(&list, env),
            "lambda" => eval_function_definition(&list),
            "map" => eval_map(&list, env),
            "filter" => eval_filter(&list, env),
            "reduce" => eval_reduce(&list, env),
            _ => eval_function_call(&s, &list, env),
        },
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = eval_obj(obj, env)?;
                match result {
                    Object::Void => {}
                    _ => new_list.push(result),
                }
            }
            Ok(Object::List(new_list))
        }
    }
}

fn eval_obj(obj: &Object, env: &mut Env) -> Result<Object, String> {
    match obj {
        Object::List(list) => eval_list(list, env),
        Object::Void => Ok(Object::Void),
        Object::Lambda(_params, _body) => Ok(Object::Void),
        Object::Bool(_) => Ok(obj.clone()),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Float(n) => Ok(Object::Float(*n)),
        Object::String(s) => Ok(Object::String(s.to_string())),
        Object::Symbol(s) => eval_symbol(s, env),
        Object::ListData(l) => Ok(Object::ListData(l.to_vec())),
    }
}

pub fn eval(program: &str, env: &mut Env) -> Result<Object, String> {
    let parsed_list = parse(program);
    if parsed_list.is_err() {
        return Err(format!("{}", parsed_list.err().unwrap()));
    }
    eval_obj(&parsed_list.unwrap(), env)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let mut env = Env::new();
        let result = eval("(+ 1 2)", &mut env).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn test_simple_sub() {
        let mut env = Env::new();
        let result = eval("(- 1.0 2)", &mut env).unwrap();
        assert_eq!(result, Object::Float(-1.0));
    }

    #[test]
    fn test_str_add() {
        let mut env = Env::new();
        let result = eval("(+ \"Raleigh\" \"Durham\")", &mut env).unwrap();
        assert_eq!(result, Object::String("RaleighDurham".to_string()));
    }

    #[test]
    fn test_str_eq_false() {
        let mut env = Env::new();
        let result = eval("(= \"Raleigh\" \"Durham\")", &mut env).unwrap();
        assert_eq!(result, Object::Bool(false));
    }

    #[test]
    fn test_str_eq_true() {
        let mut env = Env::new();
        let result = eval("(= \"Raleigh\" \"Raleigh\")", &mut env).unwrap();
        assert_eq!(result, Object::Bool(true));
    }

    #[test]
    fn test_greater_than_str() {
        let mut env = Env::new();
        let result = eval("(> \"Raleigh\" \"Durham\")", &mut env).unwrap();
        assert_eq!(result, Object::Bool(true));
    }

    #[test]
    fn test_less_than_str() {
        let mut env = Env::new();
        let result = eval("(< \"abcd\" \"abef\")", &mut env).unwrap();
        assert_eq!(result, Object::Bool(true));
    }

    #[test]
    fn test_str_with_spaces() {
        let mut env = Env::new();
        let result = eval("(+ \"Raleigh \" \"Durham\")", &mut env).unwrap();
        assert_eq!(result, Object::String("Raleigh Durham".to_string()));
    }

    #[test]
    fn test_str_with_spaces_2() {
        let mut env = Env::new();
        let program = "
        (
            (define fruits \"apples mangoes bananas \")
            (define vegetables \"carrots broccoli\")
            (+ fruits vegetables)
        )
        ";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::String(
                "apples mangoes bananas carrots broccoli".to_string()
            )])
        );
    }

    #[test]
    fn test_greater_than_int() {
        let mut env = Env::new();
        let result = eval("(> 10 20)", &mut env).unwrap();
        assert_eq!(result, Object::Bool(false));
    }

    #[test]
    fn test_less_than_int() {
        let mut env = Env::new();
        let result = eval("(< 21.0 20.0)", &mut env).unwrap();
        assert_eq!(result, Object::Bool(false));
    }

    #[test]
    fn test_modulo() {
        let mut env = Env::new();
        let result = eval("(% 21.0 20.0)", &mut env).unwrap();
        assert_eq!(result, Object::Float(1.0));
    }

    #[test]
    fn test_area_of_a_circle_float() {
        let mut env = Env::new();
        let program = "(
                        (define r 5.0)
                        (define pi 3.14)
                        (* pi (* r r))
                      )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Float((3.14 * 5.0 * 5.0) as f64)])
        );
    }

    #[test]
    fn test_area_of_a_circle() {
        let mut env = Env::new();
        let program = "(
                        (define r 10)
                        (define pi 314)
                        (* pi (* r r))
                      )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((314 * 10 * 10) as i64)])
        );
    }

    #[test]
    fn test_sqr_function() {
        let mut env = Env::new();
        let program = "(
                        (define sqr (lambda (r) (* r r))) 
                        (sqr 10)
                       )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((10 * 10) as i64)])
        );
    }

    #[test]
    fn test_map() {
        let mut env = Env::new();
        let program = "
            (
                (define sqr (lambda (r) (* r r)))
                (define l (list 1 2 3 4 5))
                (map sqr l)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::ListData(vec![
                Object::Integer(1),
                Object::Integer(4),
                Object::Integer(9),
                Object::Integer(16),
                Object::Integer(25)
            ])])
        );
    }

    #[test]
    fn test_filter() {
        let mut env = Env::new();
        let program = "
            (
                (define odd (lambda (v) (= 1 (% v 2))))
                (define l (list 1 2 3 4 5))
                (filter odd l)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::ListData(vec![
                Object::Integer(1),
                Object::Integer(3),
                Object::Integer(5)
            ])])
        );
    }

    #[test]
    fn test_reduce() {
        let mut env = Env::new();
        let program = "
            (
                (define odd (lambda (v) (= 1 (% v 2))))
                (define l (list 1 2 3 4 5))
                (reduce (lambda (x y) (| x y)) (map odd l))
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::List(vec![Object::Bool(true),]));
    }

    #[test]
    fn test_fibonaci() {
        let mut env = Env::new();
        let program = "
            (
                (define fib (lambda (n) (if (< n 2) 1 (+ (fib (- n 1)) (fib (- n 2))))))
                (fib 10)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::List(vec![Object::Integer((89) as i64)]));
    }

    #[test]
    fn test_factorial() {
        let mut env = Env::new();
        let program = "
            (
                (define fact (lambda (n) (if (< n 1) 1 (* n (fact (- n 1))))))
                (fact 5)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::List(vec![Object::Integer((120) as i64)]));
    }

    #[test]
    fn test_circle_area_function() {
        let mut env = Env::new();
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
