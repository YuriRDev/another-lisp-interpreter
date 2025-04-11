use interpreter::Interpreter;
use lexer::tokenize;
use parser::Parser;

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let input = "
(define mult 
    (lambda (a b) 
    (if (= b 1) (a) (+ a ('mult (a (+ b (-1))))))
    )
)

(define fat 
    (lambda (x) 
        (if (< x 1) 
            (1) 
            ('mult (x ('fat ((+ x (-1))))))
    )
))

    (print ('mult (2 600)))

    ";
    let tokens = tokenize(input);
    let ast = Parser::new(tokens, input).parse();
    Interpreter::interpret(ast);
}
