# REPL

## Source

[main.rs](https://github.com/vishpat/lisp-rs/blob/0.0.1/src/main.rs)

## Code Walk Through

The REPL is the simplest part of the interpreter implementation. It uses the [linefeed](https://crates.io/crates/linefeed) crate to implement the functionality.

The REPL first creates an instance of *env* to hold the global variables for the interpreter.

```
let mut env = Rc::new(RefCell::new(env::Env::new()));
```

The REPL is a simple while loop that takes one line as an input from the user, evaluates it, and prints out the evaluated object. The REPL is terminated if the user enters the **exit** keyword. 

```Rust
while let ReadResult::Input(input) = 
  			  reader.read_line().unwrap() {
    
    if input.eq("exit") {
        break;
    }
    let val = eval::eval(input.as_ref(), &mut env)?;
    match val {
        Object::Void => {}
        Object::Integer(n) => println!("{}", n),
        Object::Bool(b) => println!("{}", b),
        Object::Symbol(s) => println!("{}", s),
        Object::Lambda(params, body) => {
            println!("Lambda(");
            for param in params {
                println!("{} ", param);
            }
            println!(")");
            for expr in body {
                println!(" {}", expr);
            }
        }
        _ => println!("{}", val),
    }
}
