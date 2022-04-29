mod env;
mod eval;
mod lexer;
mod object;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let val = eval::eval("(+ 2 3)", &mut env::Env::new())?;
    println!("{:?}", val);
    Ok(())
}
