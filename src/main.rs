use interpreter::interpret;
use lexer::tokenize;
use parser::Parser;

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let input = "(print (+2 5 (if (< 3 4) (23) (0)))) (define x 123)";
    let tokens = tokenize(input);
    let ast = Parser::new(tokens, input).parse();
    interpret(ast);
}
