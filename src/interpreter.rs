use std::thread::panicking;

use crate::lexer::{Token, TokenType};

#[derive(Debug)]
pub enum Object {
    Integer(i64),
    String(String),
    Boolean(bool),
    Nil,
}

pub struct Interpreter {
    tokens: Vec<Token>,
    src: String,

    current: usize,
    length: usize,
}

enum Comparisons {
    Gt,
    Lt,
    Eq,
}

impl Interpreter {
    pub fn interpret(&mut self) {
        while self.current < self.length {
            self.consume(TokenType::LParen);
            println!("{:?}", self.eval_obj());
            self.consume(TokenType::RParen);
        }
    }

    fn eval_obj(&mut self) -> Object {
        let token = &self.tokens[self.current];
        match token._type {
            TokenType::Integer => {
                let (start, end) = token.span;
                let value = &self.src[start..end + 1];
                self.current += 1;
                Object::Integer(value.parse::<i64>().unwrap())
            }
            TokenType::LParen => {
                self.consume(TokenType::LParen);
                let eval_result = self.eval_obj();
                self.consume(TokenType::RParen);
                eval_result
            }
            TokenType::Plus => {
                self.consume(TokenType::Plus);
                self.eval_plus()
            }
            TokenType::Minus => {
                // @TODO: Implement Unary
                self.consume(TokenType::Minus);
                self.eval_minus()
            }
            TokenType::Gt => {
                self.consume(TokenType::Gt);
                self.eval_comparison(Comparisons::Gt)
            },
            TokenType::Eq => {
                self.consume(TokenType::Eq);
                self.eval_comparison(Comparisons::Eq)
            }
            TokenType::Lt => {
                self.consume(TokenType::Lt);
                self.eval_comparison(Comparisons::Lt)
            },
            TokenType::If => {
                self.consume(TokenType::If);
                self.eval_if()
            },
            _ => {
                self.current += 1;
                Object::Nil
            }
        }
    }

    fn eval_comparison(&mut self, comparison: Comparisons) -> Object {
        let objects = self.get_list();

        if objects.len() != 2 {
            panic!("Expected 2 arguments for comparison");
        }

        if let Object::Integer(left) = objects[0] {
            if let Object::Integer(right) = objects[1] {
                match comparison {
                    Comparisons::Eq => Object::Boolean(left == right),
                    Comparisons::Gt => Object::Boolean(left > right),
                    Comparisons::Lt => Object::Boolean(left < right),
                }
            } else {
                panic!("Expected number in the right side of comparison")
            }
        } else {
            panic!("Expected number in the left side of comparison")
        }
    }

    fn eval_if(&mut self) -> Object {

        Object::Nil
    }

    fn get_list(&mut self) -> Vec<Object> {
        let mut list = Vec::new();
        while self.current < self.length {
            if self.tokens[self.current]._type == TokenType::RParen {
                break;
            }
            list.push(self.eval_obj());
        }
        list
    }

    fn eval_plus(&mut self) -> Object {
        let mut sum = 0;
        let objects = self.get_list();
        for obj in objects {
            if let Object::Integer(value) = obj {
                sum += value;
            }
            // @TODO: Throw error if not integer
        }
        Object::Integer(sum)
    }

    fn eval_minus(&mut self) -> Object {
        let mut sum = 0;
        let objects = self.get_list();
        for obj in objects {
            if let Object::Integer(value) = obj {
                sum -= value;
            }
            // @TODO: Throw error if not integer
        }
        Object::Integer(sum)
    }

    fn consume(&mut self, token: TokenType) {
        if self.tokens[self.current]._type == token {
            self.current += 1;
        } else {
            panic!(
                "Expected {:?}, found {:?}",
                token, self.tokens[self.current]._type
            );
        }
    }
}

impl Interpreter {
    pub fn new(tokens: Vec<Token>, src: String) -> Interpreter {
        Interpreter {
            length: tokens.len(),
            tokens,
            current: 0,
            src,
        }
    }
}
