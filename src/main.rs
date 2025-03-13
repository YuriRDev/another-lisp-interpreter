use lexer::tokenize;
use parser::Parser;

mod lexer;
mod parser;

fn main() {
    println!("Hello, world!");
    let tokens = tokenize("(+ 1 2 (* 4 3)");
    let objects = Parser::new(tokens).parse();
    println!("{:?}", objects);
}
