# Evaluator

Now comes the most exciting part of the project. Evaluation is the final step that will produce the result for the Lisp program. At a high level, the evaluator function recursively walks the List-based structure created by the parser and evaluates each atomic object and list (recursively), combines these intermediate values, and produces the final result. 

## Source

[eval.rs](https://github.com/vishpat/lisp-rs/blob/0.0.1/src/eval.rs)

## Code Walk Through

The evaluator is implemented using the recursive *eval_obj* function. The *eval_obj* function takes the List object representing the program and the global *env* variable (a simple hashmap) as the input. The function then starts processing the List object representing the program by iterating over each element of this list 

```Rust
fn eval_obj(obj: &Object, env: &mut Rc<RefCell<Env>>) 
	-> Result<Object, String> 
{
    match obj {
        Object::Void => Ok(Object::Void),
        Object::Lambda(_params, _body) => Ok(Object::Void),
        Object::Bool(_) => Ok(obj.clone()),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Symbol(s) => eval_symbol(s, env),
        Object::List(list) => eval_list(list, env),
    }
}
```

In the case of the atomic objects such as an integer and boolean, the evaluator simply returns a copy of the object. In the case of the Void and Lambda (function objects), it returns the Void object. We will now walk through the *eval_symbol* and *eval_list* functions which implement most of the functionality of the evaluator.

### eval_symbol

Before understanding the *eval_symbol* function, it is important to understand the design of how variables are implemented for the Lisp interpreter.

The variables are just *string* labels assigned to values and they are created using the **define** keyword. Note a variable can be assigned atomic values such as integer or a boolean or it can be assigned function objects 

```Lisp
( 
  (define x 1) 
  (define sqr (lambda (r) (* r r))) 
)
```
The above Lisp code defines (or creates) two variables with the names x and sqr that represent an integer and function object respectively. Also, the scope of these variables lies within the *list object* that they are defined under. This is achieved by storing the mapping from the variable names (strings) to the objects in a map-like data structure called *Env* as shown below.

```Rust
// env.rs
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, Object>,
}
```

The interpreter creates an instance of *Env* at the start of the program to store the global variable definitions. In addition, for every function call, the interpreter creates a new instance of env and uses the new instance to evaluate the function call. This new instance of env contains the function parameters as well as a *back* pointer to the *parent* env instance from where the function is called as shown below with an example

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

![Function Call](images/env.png)   



This concept will become clearer as we will walk through the code.

The job of *eval_symbol* is to look up the Object bound to the symbol. This is done by recursively looking up in the passed *env* variable or any of its parent *env* until the *root env* of the program. 

```Rust
let val = env.borrow_mut().get(s);
if val.is_none() {
    return Err(format!("Unbound symbol: {}", s));
}
Ok(val.unwrap().clone())
```

### eval_list

The *eval_list* function is the core of the evaluator and is implemented as shown below.

```Rust
let head = &list[0];
match head {
    Object::Symbol(s) => match s.as_str() {
        "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => {
            return eval_binary_op(&list, env);
        }
        "if" => eval_if(&list, env),
        "define" => eval_define(&list, env),
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
```

This function peeks at the head of the list and if the head does not match the symbol object, it iterates all of the elements of the list recursively evaluating each element and returning a new list with the evaluated object values.

### Variable definitions

If the head of the list in the *eval_list* function matches the *define* keyword, for example

```Lisp
(define sqr (lambda (x) (* x x)))
```

the *eval_define* function calls *eval_obj* on the third argument of the list and assigns the evaluated object value to the symbol defined by the second argument in the list. The symbol and its object value are then stored in the current *env*. 

```Rust
let sym = match &list[1] {
    Object::Symbol(s) => s.clone(),
    _ => return Err(format!("Invalid define")),
};
let val = eval_obj(&list[2], env)?;
env.borrow_mut().set(&sym, val);
```

In the example above the symbol *sqr* and the function object representing the lambda will be stored in the current *env*. Once the function *sqr* has been defined in this manner, any latter code can access the corresponding function object by looking up the symbol *sqr* in *env*.


### Binary operations

If the head of the list in the *eval_list* function matches a binary operator, the list is evaluated on the basis of the type of the binary operator, for example 

```Lisp
(+ x y)
```
the *eval_binary_op* function calls the *eval_obj* on the second and third element of the list and performs the binary **sum** operation on the evaluated values.

### If statement

If the head of the list in the *eval_list* function matches the *if* keyword, for example

```Lisp
(if (> x y) (x) (y))
```

the *eval_if* function calls **eval_obj** on the second element of the list and depending upon whether the evaluated value is true or false, calls the eval_obj on either the third or fourth element of the list and returns the value

```
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
```


### Lambda
As mentioned earlier, the *lambda* (or function) object consists of two vectors

```Rust
Lambda(Vec<String>, Vec<Object>)
```

If the head of the list in the *eval_list* function matches the *lambda* keyword, for example

```Lisp
(lambda (x) (* x x))
```
the *eval_function_definition* function evaluates the second element of the list as a vector of parameter names. 

```Rust
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
```

The third element of the list is simply cloned as the function body.

```Rust
let body = match &list[2] {
    Object::List(list) => list.clone(),
    _ => return Err(format!("Invalid lambda")),
};
```

```Rust
Ok(Object::Lambda(params, body))
``` 

The evaluated parameter and body vector are returned as the *lambda* object

### Function Call

If the head of the list is a Symbol object and it does not match any of the aforementioned keywords or binary operators, the interpreter assumes that the Symbol object maps to a Lambda (function object). An example of the function call in Lisp is as follows

```Lisp
(find_max a b c)
```

To evaluate this list the *eval_function_call* function is called. This function first performs the lookup for the function object using the function name, *find_max* in the case of this example.

```Rust
let lamdba = env.borrow_mut().get(s);
if lamdba.is_none() {
    return Err(format!("Unbound symbol: {}", s));
}
```

If the function object is found, a new *env* object is created. This new *env* object has a pointer to the parent *env* object. This is required to get the values of the variables not defined in the scope of the function.  

```Rust
let mut new_env = Rc::new(
    			  RefCell::new(
    			  Env::extend(env.clone())));

```

The next step in evaluating the function call requires preparing the function parameters. This is done by iterating over the remainder of the list and evaluating each parameter. The parameter name and the evaluated object are then set in the new *env* object.

```Rust
for (i, param) in params.iter().enumerate() {
    let val = eval_obj(&list[i + 1], env)?;
    new_env.borrow_mut().set(param, val);
}
```

Finally, the function body is evaluated by passing the new_env, which contains the parameters to the function

```Rust
let new_body = body.clone();
return eval_obj(&Object::List(new_body), &mut new_env);
```


 