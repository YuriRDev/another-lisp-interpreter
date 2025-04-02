use interpreter::Interpreter;
use lexer::tokenize;
use parser::Parser;

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let input = "
    (print (+2 5 (if (< 3 4) (23) (0)))) 
    (print (define x (123)))
    (print ( x ))
    (define x (+x 1))
    (print ( x ))
    ";
    let tokens = tokenize(input);
    let ast = Parser::new(tokens, input).parse();
    Interpreter::interpret(ast);
}
