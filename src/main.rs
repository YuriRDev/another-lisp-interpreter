use interpreter::Interpreter;
use lexer::tokenize;
use parser::Parser;

mod lexer;
mod parser;
mod interpreter;

fn main() {
    let input = "(print (+2 5 (if (< 3 4) (23) (0))))";
    let tokens = tokenize(input);
    let ast = Parser::new(tokens, input).parse();
    let interpret = Interpreter::new(ast).interpret();
}
