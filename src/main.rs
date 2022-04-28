mod lexer;
mod object;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokens = lexer::tokenize("(define rx1 (+ 1 2))")?;
    let parsed_list = parser::parse_list(&mut tokens.into_iter().rev().collect())?;
    println!("{:?}", parsed_list);
    Ok(())
}
