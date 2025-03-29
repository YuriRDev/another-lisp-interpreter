use interpreter::Interpreter;
use lexer::tokenize;

mod interpreter;
mod lexer;

fn main() {
    let input = "
        (> 3 (+ 2 2))
        (= 3 (+ 2 2))
        (< 3 (< 2 1))
        ";
    let tokens = tokenize(input);
    Interpreter::new(tokens, input.to_string()).interpret();
}
