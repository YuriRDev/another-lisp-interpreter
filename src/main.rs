use interpreter::Interpreter;
use lexer::tokenize;
use parser::Parser;

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let input = "
    (print (\"What's the first number? \"))
    (define x (readn))
    (print (\"What's the second number? \"))
    (define y (readn))
    (define dif (+ x (-y)))
    (print (\"The difference is: \"))
    (print (dif))
    ";
    let tokens = tokenize(input);
    let ast = Parser::new(tokens, input).parse();
    Interpreter::interpret(ast);
}
