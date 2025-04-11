use interpreter::Interpreter;
use lexer::tokenize;
use parser::Parser;

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let input = "
    (define plus (lambda (a b) (+ a b)))
    (print (plus))
    (define x ('plus (4 6)))
    (print (x))
    ";
    let tokens = tokenize(input);
    let ast = Parser::new(tokens, input).parse();
    Interpreter::interpret(ast);
}
