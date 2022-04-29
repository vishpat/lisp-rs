mod env;
mod eval;
mod lexer;
mod object;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed_list = parser::parse("(+ (* 2 6) (+ 2 3))")?;
    let val = eval::eval(&parsed_list, &mut env::Env::new())?;
    println!("{:?}", val);
    Ok(())
}
