use crate::parser::{AST, AST};

#[derive(Debug)]
pub enum Object {
    Boolean(bool),
    Number(i64),
    String(String),
    Void,
}

pub struct Interpreter {
    ast: Vec<AST>,
}

impl Interpreter {
    pub fn interpret(&self) {
        self.evaluate(&self.ast[0]);
    }

    fn evaluate(&self, ast: &AST) -> Object {
        match ast._type {
            AST::Number(e) => Object::Number(e),
            AST::Boolean(e) => Object::Boolean(e),
            AST::Print => {
                println!("{:?}", self.evaluate(&ast.children[0]));
                Object::Void
            }

            AST::If => {
                let objects = &ast.children;

                if let Object::Boolean(a) = self.evaluate(&objects[0]) {
                    if a {
                        self.evaluate(&objects[1])
                    } else {
                        self.evaluate(&objects[2])
                    }

                } else {
                    unreachable!("Treated at the parser");
                }
            }

            AST::EQop => {
                let obj = &ast.children;
                if let Object::Number(left) = &self.evaluate(&obj[0]) {
                    if let Object::Number(right) = &self.evaluate(&obj[1]) {
                        Object::Boolean(left == right)
                    } else {
                        unreachable!("Treated on parser");
                    }
                } else {
                    unreachable!("Treated on parser");
                }
            }

            AST::LTop => {
                let obj = &ast.children;
                if let Object::Number(left) = &self.evaluate(&obj[0]) {
                    if let Object::Number(right) = &self.evaluate(&obj[1]) {
                        Object::Boolean(left < right)
                    } else {
                        unreachable!("Treated on parser");
                    }
                } else {
                    unreachable!("Treated on parser");
                }
            }

            AST::GTop => {
                let obj = &ast.children;
                if let Object::Number(left) = &self.evaluate(&obj[0]) {
                    if let Object::Number(right) = &self.evaluate(&obj[1]) {
                        Object::Boolean(left > right)
                    } else {
                        unreachable!("Treated on parser");
                    }
                } else {
                    unreachable!("Treated on parser");
                }
            }

            AST::Plus => {
                let mut sum = 0;
                for c in &ast.children {
                    match self.evaluate(c) {
                        Object::Number(c) => sum += c,
                        _ => panic!("Expected number in plus operation"),
                    }
                }
                Object::Number(sum)
            }
            _ => Object::Void,
        }
    }
}

impl Interpreter {
    pub fn new(ast: Vec<AST>) -> Interpreter {
        Interpreter { ast }
    }
}
