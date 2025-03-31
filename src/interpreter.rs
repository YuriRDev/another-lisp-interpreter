use crate::parser::{ASTType, AST};

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
            ASTType::Number(e) => Object::Number(e),
            ASTType::Boolean(e) => Object::Boolean(e),
            ASTType::Print => {
                println!("{:?}", self.evaluate(&ast.children[0]));
                Object::Void
            }

            ASTType::If => {
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

            ASTType::EQop => {
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

            ASTType::LTop => {
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

            ASTType::GTop => {
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

            ASTType::Plus => {
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
