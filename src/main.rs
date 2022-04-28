mod lexer;
mod object;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed_list = parser::parse("(begin (define x 1) (define y 2) (+ x y))")?;
    println!("{:?}", parsed_list);
    Ok(())
}
