use std::{collections::HashMap, io};

use crate::parser::{ArithmeticOp, BinaryOp, InputType, AST};

#[derive(Debug, Clone)]
pub enum Object {
    Boolean(bool),
    Number(i64),
    String(String),
    Void,
}

type Scope = HashMap<String, Object>;

pub struct Interpreter {
    scope: Scope,
}

impl Interpreter {
    fn evaluate(&mut self, ast: &AST) -> Object {
        match ast {
            AST::Number(e) => Object::Number(*e),
            AST::Boolean(e) => Object::Boolean(*e),
            AST::Print(children) => {
                let child = self.evaluate(children);
                println!("→ {}", &self.print(child));
                Object::Void
            }

            AST::If(condition, _true, _false) => {
                if let Object::Boolean(a) = self.evaluate(condition) {
                    if a {
                        self.evaluate(_true)
                    } else {
                        self.evaluate(_false)
                    }
                } else {
                    unreachable!("Treated at the parser");
                }
            }

            AST::Binary(op, left, right) => {
                if let Object::Number(left) = &self.evaluate(left) {
                    if let Object::Number(right) = &self.evaluate(right) {
                        return Object::Boolean(match op {
                            BinaryOp::Eq => left == right,
                            BinaryOp::Lt => left < right,
                            BinaryOp::Gt => left > right,
                        });
                    }
                }
                unreachable!("Treated on parser");
            }

            AST::Arithmetic(op, list) => {
                let mut sum = 0;
                for c in list {
                    if let Object::Number(r) = self.evaluate(c) {
                        match op {
                            ArithmeticOp::Minus => sum -= r,
                            ArithmeticOp::Plus => sum += r,
                        }
                    }
                }
                Object::Number(sum)
            }
            AST::String(e) => Object::String(e.clone()),
            AST::Define(_x, _y) => {
                let evaluated = self.evaluate(_y);
                self.scope.insert(_x.to_string(), evaluated);
                Object::Void
            }
            AST::Identifier(name) => match self.scope.get(name) {
                // @TODO: Remove this clone.
                Some(data) => data.clone(),
                None => panic!("Undefined variable"),
            },
            AST::Input(_type) => {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");

                match _type {
                    InputType::String => Object::String(input.trim().to_string()),
                    _ => {
                        let number: i64 = input.trim().parse().expect("Invalid number");
                        Object::Number(number)
                    }
                }
            }
        }
    }

    fn print(&self, obj: Object) -> String {
        match obj {
            Object::Number(e) => format!("{}", e),
            Object::String(e) => e,
            Object::Boolean(e) => format!("{}", e),
            Object::Void => "_void".to_string(),
        }
    }
}

impl Interpreter {
    pub fn interpret(asts: Vec<AST>) {
        let mut interpreter = Interpreter {
            scope: HashMap::new(),
        };
        for ast in asts {
            interpreter.evaluate(&ast);
        }
    }
}
