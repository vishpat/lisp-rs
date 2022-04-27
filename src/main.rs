mod lexer;

fn main() {
    let tokens = lexer::tokenize("(define rx1 (+ 1 2))").unwrap_or(vec![]);
    for token in tokens {
        println!("{}", token);
    }
}
