# Evaluator

Now comes the most exciting part of the project. Evaluation is the final step that will produce the result for the Lisp program. At a high level, the evaluator function recursively walks the List-based structure created by the parser and evaluates each atomic object and list (recursively), combines these results, and produces the final result. 

## Source

**eval.rs**

## Code Walk Through

Before walking through the evaluator code, it is important to explain the design of how variables and functions are implemented for the interpreter. 

### Variables

The variables are just *string* symbols assigned to values and they are created using the **define** keyword. Note a variable can be assigned atomic values such as integer or a boolean as well as function objects 

```Lisp
( 
  (define x 1) 
  (define sqr (lambda (r) (* r r))) 
)
```
This defines (or creates) two variables with the names x and sqr that represent an integer and string object respectively. Also, the scope of these variables lies within the *list object* that they are defined under. This is achieved by storing the mapping from the variable names (strings) to the objects in a map-like data structure as shown below.

```Rust
// env.rs
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, Object>,
}
```

The interpreter creates an instance of *Env* at the start of the program to store all of the variable definitions. In addition for every function call, the interpreter creates a new instance of env and uses the new instance to evaluate the function call. This new instance of env contains the function parameters as well as a *back* pointer to the *parent* env instance from where the function is called as shown below with an example

```Lisp
(
	(define m 10)
	(define n 12)
	(define K 100)
	
	(define func1 (lambda (x) (+ x K)))
	(define func2 (lambda (x) (- x K)))
	
	(func1 m)
	(func2 n)
)
```

![Function Call](images/function_call.png)   

This concept will become clearer as we will walk through the code


### Function Objects

Functions are represented by the Lambda Object which consists of two Lists (vectors). 

```
Lambda(Vec<String>, Vec<Object>)
```

The first list is the list of parameters while the second list is the list of instructions forming the function definition. 

### Evaluator

The evaluator is implemented using the recursive *eval_obj* function. This function takes the List object representing the program and the global *env* variable as the input. The function then starts processing the List object representing the program by iterating over each element of this list as shown below.

```
match obj {
    Object::List(list) => {
        let head = &list[0];
        match head {
            Object::Symbol(s) => match s.as_str() {
                "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => {
                    return eval_binary_op(&list, env);
                }
                "define" => eval_define(&list, env),
                "if" => eval_if(&list, env),
                "lambda" => eval_function_definition(&list),
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
    Object::Void => Ok(Object::Void),
    Object::Lambda(_params, _body) => Ok(Object::Void),
    Object::Bool(_) => Ok(obj.clone()),
    Object::Integer(n) => Ok(Object::Integer(*n)),
    Object::Symbol(s) => {
        let val = env.borrow_mut().get(s);
        if val.is_none() {
            return Err(format!("Unbound symbol: {}", s));
        }
        return Ok(val.unwrap().clone());
    }
}

```
 

