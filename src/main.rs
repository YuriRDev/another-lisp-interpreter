use interpreter::{Interpreter, Scope};
use lexer::tokenize;
use parser::Parser;
use std::{
    collections::HashMap,
    env, fs,
    io::{self},
};

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        let content = fs::read_to_string(&args[1]).expect("Couldn't open the file");
        let tokens = tokenize(&content);
        let ast = Parser::new(tokens, &content).parse();
        Interpreter::interpret(ast);
    } else {
        let mut input = String::new();
        // The scope is passed in every new line. 
        // Just like an inheritance (literally). 
        // Not sure if the REPL concept uses this to 
        // preserve context. But is good enough
        let mut context: Scope = HashMap::new();
        loop {
            print!("→ ");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let tokens = tokenize(&input);
            let ast = Parser::new(tokens, &input).parse();
            context = Interpreter::interpret_repl(ast, context);
            input.clear();
        }
    }
}
