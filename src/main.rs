use interpreter::Interpreter;
use lexer::tokenize;
use parser::Parser;

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let input = "
    (print (+ 1 2))
    (print (- 1 2))
    (print (+ 1 (+2 3)))
    ";
    let tokens = tokenize(input);
    let ast = Parser::new(tokens, input).parse();
    Interpreter::interpret(ast);
}
