use crate::env::*;
use crate::object::*;
use crate::parser::*;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

fn print_list(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let mut new_list = Vec::new();

    for obj in list[1..].iter() {
        new_list.push(eval_obj(obj, env)?);
    }
    for obj in new_list.iter() {
        print!("{} ", obj);
    }
    println!();
    Ok(Object::Void)
}

fn eval_car(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let l = eval_obj(&list[1], env)?;
    match l {
        Object::ListData(list) => Ok(list[0].clone()),
        _ => Err(format!("{} is not a list", l)),
    }
}

fn eval_cdr(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let l = eval_obj(&list[1], env)?;
    let mut new_list = vec![];
    match l {
        Object::ListData(list) => {
            for obj in list[1..].iter() {
                new_list.push(obj.clone());
            }
            Ok(Object::ListData(new_list))
        }
        _ => Err(format!("{} is not a list", l)),
    }
}

fn eval_length(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let obj = eval_obj(&list[1], env)?;
    match obj {
        Object::List(list) => Ok(Object::Integer(list.len() as i64)),
        Object::ListData(list) => Ok(Object::Integer(list.len() as i64)),
        _ => Err(format!("{} is not a list", obj)),
    }
}

fn eval_is_null(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let obj = eval_obj(&list[1], env)?;
    match obj {
        Object::List(list) => Ok(Object::Bool(list.is_empty())),
        Object::ListData(list) => Ok(Object::Bool(list.is_empty())),
        _ => Err(format!("{} is not a list", obj)),
    }
}

fn eval_binary_op(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err("Invalid number of arguments for infix operator".to_string());
    }
    let operator = list[0].clone();
    let left = &eval_obj(&list[1].clone(), env)?;
    let right = &eval_obj(&list[2].clone(), env)?;
    match operator {
        Object::BinaryOp(s) => match s.as_str() {
            "+" => match (left, right) {
                (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l + r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l + r)),
                (Object::Integer(l), Object::Float(r)) => Ok(Object::Float(*l as f64 + r)),
                (Object::Float(l), Object::Integer(r)) => Ok(Object::Float(l + *r as f64)),
                (Object::String(l), Object::String(r)) => Ok(Object::String(l.to_owned() + r)),
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
                    Ok(Object::Bool(l.cmp(r) == Ordering::Less))
                }
                _ => Err(format!("Invalid types for < operator {} {}", left, right)),
            },
            ">" => match (left, right) {
                (Object::Integer(l), Object::Integer(r)) => Ok(Object::Bool(l > r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Bool(l > r)),
                (Object::Integer(l), Object::Float(r)) => Ok(Object::Bool(*l as f64 > *r)),
                (Object::Float(l), Object::Integer(r)) => Ok(Object::Bool(l > &(*r as f64))),
                (Object::String(l), Object::String(r)) => {
                    Ok(Object::Bool(l.cmp(r) == Ordering::Greater))
                }
                _ => Err(format!("Invalid types for > operator {} {}", left, right)),
            },
            "=" => match (left, right) {
                (Object::Integer(l), Object::Integer(r)) => Ok(Object::Bool(l == r)),
                (Object::String(l), Object::String(r)) => Ok(Object::Bool(l == r)),
                _ => Err(format!("Invalid types for = operator {} {}", left, right)),
            },
            "!=" => match (left, right) {
                (Object::Integer(l), Object::Integer(r)) => Ok(Object::Bool(l != r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Bool(l != r)),
                (Object::Integer(l), Object::Float(r)) => Ok(Object::Bool(*l as f64 != *r)),
                (Object::Float(l), Object::Integer(r)) => Ok(Object::Bool(*l != (*r) as f64)),
                (Object::String(l), Object::String(r)) => {
                    Ok(Object::Bool(l.cmp(r) != Ordering::Equal))
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
        _ => Err("Operator must be a symbol".to_string()),
    }
}

fn eval_begin(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let mut result = Object::Void;
    let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));

    for obj in list[1..].iter() {
        result = eval_obj(obj, &mut new_env)?;
    }
    Ok(result)
}

fn eval_let(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let mut result = Object::Void;
    let bindings_env = Rc::new(RefCell::new(Env::new()));

    if list.len() < 3 {
        return Err("Invalid number of arguments for let".to_string());
    }

    let bindings = match list[1].clone() {
        Object::List(bindings) => bindings,
        _ => return Err("Invalid bindings for let".to_string()),
    };

    for binding in bindings.iter() {
        let binding = match binding {
            Object::List(binding) => binding,
            _ => return Err("Invalid binding for let".to_string()),
        };

        if binding.len() != 2 {
            return Err("Invalid binding for let".to_string());
        }

        let name = match binding[0].clone() {
            Object::Symbol(name) => name,
            _ => return Err("Invalid binding for let".to_string()),
        };

        let value = eval_obj(&binding[1], env)?;
        bindings_env.borrow_mut().set(name.as_str(), value);
    }

    println!("let arguments {:?}", bindings_env);
    let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
    new_env.borrow_mut().update(bindings_env);

    for obj in list[2..].iter() {
        result = eval_obj(obj, &mut new_env)?;
    }
    Ok(result)
}

fn eval_define(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err("Invalid number of arguments for define".to_string());
    }

    let sym = match &list[1] {
        Object::Symbol(s) => s.clone(),
        Object::List(l) => {
            let name = match &l[0] {
                Object::Symbol(s) => s.clone(),
                _ => return Err("Invalid symbol for define".to_string()),
            };
            let params = Object::List(Rc::new(l[1..].to_vec()));
            let body = list[2].clone();
            let lambda = eval_function_definition(&[Object::Void, params, body], env)?;
            env.borrow_mut().set(&name, lambda);
            return Ok(Object::Void);
        }
        _ => return Err("Invalid define".to_string()),
    };
    let val = eval_obj(&list[2], env)?;
    env.borrow_mut().set(&sym, val);
    Ok(Object::Void)
}

fn eval_list_data(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let mut new_list = Vec::new();

    for obj in list[1..].iter() {
        new_list.push(eval_obj(obj, env)?);
    }
    Ok(Object::ListData(new_list))
}

fn eval_cond(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() < 2 {
        return Err("Invalid number of arguments for cond".to_string());
    }

    println!("cond {:?}", list);

    for l in list[1..].iter() {
        match l {
            Object::List(list) => {
                if list.len() != 2 {
                    return Err(format!("Invalid cond clause {:?}", list));
                }

                if list[0] == Object::Keyword("else".to_string()) {
                    return eval_else(list, env);
                }

                let cond = eval_obj(&list[0], env)?;
                let cond_val = match cond {
                    Object::Bool(b) => b,
                    _ => return Err(format!("Condition must be a boolean {:?}", cond)),
                };
                if cond_val {
                    return eval_obj(&list[1], env);
                }
            }
            _ => return Err("Invalid cond clause".to_string()),
        }
    }

    Err("No cond clause matched".to_string())
}

fn eval_else(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 2 {
        return Err(format!("Invalid number of arguments for else {:?}", list));
    }

    eval_obj(&list[1], env)
}

fn eval_function_definition(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let params = match &list[1] {
        Object::List(list) => {
            let mut params = Vec::new();
            for param in (*list).iter() {
                match param {
                    Object::Symbol(s) => params.push(s.clone()),
                    _ => return Err(format!("Invalid lambda parameter {:?}", param)),
                }
            }
            params
        }
        _ => return Err("Invalid lambda".to_string()),
    };

    let body = match &list[2] {
        Object::List(list) => list.clone(),
        _ => return Err("Invalid lambda".to_string()),
    };
    Ok(Object::Lambda(params, Rc::new(body.to_vec()), env.clone()))
}

fn eval_symbol(s: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let val = match s {
        "#t" => return Ok(Object::Bool(true)),
        "#f" => return Ok(Object::Bool(false)),
        "#nil" => return Ok(Object::Void),
        _ => env.borrow_mut().get(s),
    };

    if val.is_none() {
        return Err(format!("Unbound symbol: {}", s));
    }

    Ok(val.unwrap().clone())
}

fn eval_keyword(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let head = &list[0];
    match head {
        Object::Keyword(s) => match s.as_str() {
            "define" => eval_define(list, env),
            "begin" => eval_begin(list, env),
            "let" => eval_let(list, env),
            "list" => eval_list_data(list, env),
            "print" => print_list(list, env),
            "lambda" => eval_function_definition(list, env),
            "car" => eval_car(list, env),
            "cdr" => eval_cdr(list, env),
            "length" => eval_length(list, env),
            "null?" => eval_is_null(list, env),
            "cond" => eval_cond(list, env),
            _ => Err(format!("Unknown keyword: {}", s)),
        },
        _ => Err(format!("Invalid keyword: {}", head)),
    }
}

fn eval_obj(obj: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let mut current_obj = Box::new(obj.clone());
    let mut current_env = env.clone();
    loop {
        match *current_obj {
            Object::List(list) => {
                let head = &list[0];
                match head {
                    Object::BinaryOp(_op) => {
                        return eval_binary_op(&list, &mut current_env);
                    }
                    Object::Keyword(_keyword) => {
                        if _keyword == "if" {
                            if list.len() != 4 {
                                return Err(
                                    "Invalid number of arguments for if statement".to_string()
                                );
                            }

                            let cond_obj = eval_obj(&list[1], &mut current_env)?;
                            let cond = match cond_obj {
                                Object::Bool(b) => b,
                                _ => return Err("Condition must be a boolean".to_string()),
                            };

                            if cond {
                                current_obj = Box::new(list[2].clone());
                            } else {
                                current_obj = Box::new(list[3].clone());
                            }
                            continue;
                        } else {
                            return eval_keyword(&list, &mut current_env);
                        }
                    }
                    Object::Lambda(params, body, func_env) => {
                        let new_env = Rc::new(RefCell::new(Env::extend(func_env.clone())));
                        for (i, param) in params.iter().enumerate() {
                            let val = eval_obj(&list[i + 1], &mut current_env)?;
                            new_env.borrow_mut().set(param, val);
                        }
                        current_obj = Box::new(Object::List(body.clone()));
                        current_env = new_env;
                        continue;
                    }
                    Object::Symbol(s) => {
                        let lamdba = current_env.borrow_mut().get(s);
                        if lamdba.is_none() {
                            return Err(format!("Unbound function: {}", s));
                        }

                        let func = lamdba.unwrap();
                        match func {
                            Object::Lambda(params, body, func_env) => {
                                let new_env = Rc::new(RefCell::new(Env::extend(func_env.clone())));
                                for (i, param) in params.iter().enumerate() {
                                    let val = eval_obj(&list[i + 1], &mut current_env)?;
                                    new_env.borrow_mut().set(param, val);
                                }
                                current_obj = Box::new(Object::List(body));
                                current_env = new_env.clone();
                                continue;
                            }
                            _ => return Err(format!("Not a lambda: {} {:?}", s, func)),
                        }
                    }
                    _ => {
                        let mut new_list = Vec::new();
                        for obj in (*list).iter() {
                            let result = eval_obj(obj, &mut current_env)?;
                            match result {
                                Object::Void => {}
                                _ => new_list.push(result),
                            }
                        }
                        let head = &new_list[0];
                        match head {
                            Object::Lambda(_, _, _) => {
                                return eval_obj(
                                    &Object::List(Rc::new(new_list)),
                                    &mut current_env,
                                );
                            }
                            _ => {
                                return Ok(Object::List(Rc::new(new_list)));
                            }
                        }
                    }
                }
            }
            Object::Symbol(s) => {
                return eval_symbol(&s, &mut current_env);
            }
            Object::Void => return Ok(Object::Void),
            Object::Lambda(_params, _body, _func_env) => return Ok(Object::Void),
            Object::Bool(_) => return Ok(obj.clone()),
            Object::Integer(n) => return Ok(Object::Integer(n)),
            Object::Float(n) => return Ok(Object::Float(n)),
            Object::String(s) => return Ok(Object::String(s.to_string())),
            Object::ListData(l) => return Ok(Object::ListData(l.to_vec())),
            _ => return Err(format!("Invalid object: {:?}", obj)),
        }
    }
}

pub fn eval(program: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
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
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(+ 1 2)", &mut env).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn test_simple_sub() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(- 1.0 2)", &mut env).unwrap();
        assert_eq!(result, Object::Float(-1.0));
    }

    #[test]
    fn test_str_add() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(+ \"Raleigh\" \"Durham\")", &mut env).unwrap();
        assert_eq!(result, Object::String("RaleighDurham".to_string()));
    }

    #[test]
    fn test_str_eq_false() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(= \"Raleigh\" \"Durham\")", &mut env).unwrap();
        assert_eq!(result, Object::Bool(false));
    }

    #[test]
    fn test_str_eq_true() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(= \"Raleigh\" \"Raleigh\")", &mut env).unwrap();
        assert_eq!(result, Object::Bool(true));
    }

    #[test]
    fn test_greater_than_str() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(> \"Raleigh\" \"Durham\")", &mut env).unwrap();
        assert_eq!(result, Object::Bool(true));
    }

    #[test]
    fn test_less_than_str() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(< \"abcd\" \"abef\")", &mut env).unwrap();
        assert_eq!(result, Object::Bool(true));
    }

    #[test]
    fn test_str_with_spaces() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(+ \"Raleigh \" \"Durham\")", &mut env).unwrap();
        assert_eq!(result, Object::String("Raleigh Durham".to_string()));
    }

    #[test]
    fn test_str_with_spaces_2() {
        let mut env = Rc::new(RefCell::new(Env::new()));
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
            Object::List(Rc::new(vec![Object::String(
                "apples mangoes bananas carrots broccoli".to_string()
            )]))
        );
    }

    #[test]
    fn test_greater_than_int() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(> 10 20)", &mut env).unwrap();
        assert_eq!(result, Object::Bool(false));
    }

    #[test]
    fn test_less_than_int() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(< 21.0 20.0)", &mut env).unwrap();
        assert_eq!(result, Object::Bool(false));
    }

    #[test]
    fn test_modulo() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(% 21.0 20.0)", &mut env).unwrap();
        assert_eq!(result, Object::Float(1.0));
    }

    #[test]
    fn test_area_of_a_circle_float() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (begin
                (define r 5.0)
                (define pi 3.14)
                (* pi (* r r))
            )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Float((3.14 * 5.0 * 5.0) as f64));
    }

    #[test]
    fn test_area_of_a_circle() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (begin
                (define r 10)
                (define pi 314)
                (* pi (* r r))
            )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((314 * 10 * 10) as i64));
    }

    #[test]
    fn test_sqr_function() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (begin
                (define sqr (lambda (r) (* r r))) 
                (sqr 10)
            )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((10 * 10) as i64));
    }

    #[test]
    fn test_fibonaci() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (begin
                (define fib (lambda (n) 
                    (if (< n 2) 1 
                        (+ (fib (- n 1)) 
                            (fib (- n 2))))))
                (fib 10)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((89) as i64));
    }

    #[test]
    fn test_factorial() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (begin
                (define fact (lambda (n) (if (< n 1) 1 (* n (fact (- n 1))))))
                (fact 5)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((120) as i64));
    }

    #[test]
    fn test_circle_area_no_lambda() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (begin
                (define pi 314)
                (define r 10)
                (define (sqr r) (* r r))
                (define (area r) (* pi (sqr r)))
                (area r)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((314 * 10 * 10) as i64));
    }

    #[test]
    fn test_circle_area_function() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (begin
                (define pi 314)
                (define r 10)
                (define sqr (lambda (r) (* r r)))
                (define area (lambda (r) (* pi (sqr r))))
                (area r)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((314 * 10 * 10) as i64));
    }

    #[test]
    fn test_tail_recursion() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (begin
                (define sum-n 
                   (lambda (n a) 
                      (if (= n 0) a 
                          (sum-n (- n 1) (+ n a)))))
                (sum-n 500 0)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((125250) as i64));
    }

    #[test]
    fn test_tail_recursive_factorial() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (begin
                (define fact 
                    (lambda (n a) 
                      (if (= n 1) a 
                        (fact (- n 1) (* n a)))))
                        
                (fact 10 1)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((3628800) as i64));
    }

    #[test]
    fn test_closure1() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (begin
                (define add-n 
                   (lambda (n) 
                      (lambda (a) (+ n a))))
                (define add-5 (add-n 5))
                (add-5 10)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((15) as i64));
    }

    #[test]
    fn test_tail_recursive_fibonnaci() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (begin
                (define fib
                  (lambda (n a b) 
                     (if (= n 0) a 
                       (if (= n 1) b 
                          (fib (- n 1) b (+ a b))))))
                  
                (fib 10 0 1)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((55) as i64));
    }

    #[test]
    fn test_inline_lambda() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
        (begin
            ((lambda (x y) (+ x y)) 10 20)
        )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((30) as i64));
    }

    #[test]
    fn test_car() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
        (begin
            (car (list 1 2 3))
        )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((1) as i64));
    }

    #[test]
    fn test_cdr() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
        (begin
            (cdr (list 1 2 3))
        )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::ListData(vec![Object::Integer(2), Object::Integer(3),])
        );
    }

    #[test]
    fn test_length() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
        (begin
            (length (list 1 2 3))
        )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((3) as i64));
    }

    #[test]
    fn test_sum_list_of_integers() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
        (begin
            (define sum-list 
                (lambda (l) 
                    (if (null? l) 0 
                        (+ (car l) (sum-list (cdr l))))))
            (sum-list (list 1 2 3 4 5))
        )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer(15));
    }

    #[test]
    fn test_function_application() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
        (begin
            (define (double value) 
                (* 2 value))
            (define (apply-twice fn value) 
                (fn (fn value)))
        
            (apply-twice double 5)
        )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((20) as i64));
    }

    #[test]
    fn test_begin_scope_test() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
        (begin
            (define a 10)
            (define b 20)
            (define c 30)
            (begin
                (define a 20)
                (define b 30)
                (define c 40)
                (list a b c)
            )
        )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::ListData(vec![
                Object::Integer(20),
                Object::Integer(30),
                Object::Integer(40),
            ])
        );
    }

    #[test]
    fn test_begin_scope_test_2() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
        (begin 
            (define x 10)
            (begin
                (define x 20)
                x 
            )
            x
        )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer((10) as i64));
    }

    #[test]
    fn test_cond_1() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (cond ((> 2 1) 5) 
                  ((< 2 1) 10) 
                  (else 15)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer(5));
    }

    #[test]
    fn test_cond_2() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (cond ((> 1 2) 5) 
                  ((< 1 2) 10) 
                  (else 15)
        )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer(10));
    }

    #[test]
    fn test_cond_3() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (cond ((> 1 2) 5) 
                  ((< 1 0) 10) 
                  (else 15)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer(15));
    }

    #[test]
    fn test_let_1() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
        (begin
            (let ((a 10) (b 20))
                (list a b)
            )
        )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::ListData(vec![Object::Integer(10), Object::Integer(20),])
        );
    }

    #[test]
    fn test_let_2() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
        (begin
            (define a 100)
            (let ((a 10) (b 20))
                (list a b)
            )
            a
        )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer(100));
    }

    #[test]
    fn test_let_3() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (let ((x 2) (y 3))
                (let ((x 7)
                      (z (+ x y)))
                    (* z x))) 
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::Integer(35));
    }
}
