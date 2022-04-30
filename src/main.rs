mod env;
mod eval;
mod lexer;
mod object;
mod parser;

use std::cell::RefCell;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut env = Rc::new(RefCell::new(env::Env::new()));
    let val = eval::eval("(+ 2 3)", &mut env)?;
    println!("{:?}", val);
    Ok(())
}
