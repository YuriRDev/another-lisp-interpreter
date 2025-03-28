use lexer::tokenize;

mod lexer;

fn main() {
    let tokens = tokenize(
        "
        (define test 10)
        ",
    );
    println!("{:?}", tokens);
}
