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

The interpreter creates an instance of *Env* at the start of the program to store all of the variable definitions. In addition for every function call, the interpreter creates an instance of env and uses the new instance to evaluate the function call. This new instance of env contains the function parameters as well as a back pointer to the env instance from where the function is called as shown below

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


### Functions


