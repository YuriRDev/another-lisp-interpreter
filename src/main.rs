use evaluator::Eval;
use lexer::tokenize;
use parser::Parser;

mod evaluator;
mod lexer;
mod parser;

fn main() {
    let tokens = tokenize(
        "
        (
            ( define test 10 )
            ( define func (lambda ( a b c x ) ( + a b c x test  ) ) )
            ( print ( func test 2 3 4 ) )
        )
        ",
    );
    let object = Parser::new(tokens).parse();
    println!("{:?}", object);
    Eval::new().eval(object);
}
