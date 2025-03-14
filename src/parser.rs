use std::collections::VecDeque;

use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
    String(String),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn parse(&mut self) -> Object {
        let token = self.tokens.pop_front();
        if token != Some(Token::LParen) {
            panic!("Expected LParen, found {:?}", token);
        };
        if let Some(objects) = self.parse_expression() {
            if !self.tokens.is_empty() {
                println!("Expected EOF, but there's still tokens left.");
                panic!("force stop");
            }
            return objects;
        }
        panic!("force stop");
    }

    fn parse_expression(&mut self) -> Option<Object> {
        let mut list: Vec<Object> = Vec::new();
        while !self.tokens.is_empty() {
            let token = self.tokens.pop_front();
            match token {
                Some(Token::Integer(value)) => list.push(Object::Integer(value)),
                Some(Token::Symbol(value)) => list.push(Object::Symbol(value)),
                Some(Token::String(value)) => list.push(Object::String(value)),
                Some(Token::LParen) => {
                    if let Some(sub_list) = self.parse_expression() {
                        list.push(sub_list);
                    }
                }
                Some(Token::RParen) => {
                    return Some(Object::List(list));
                }
                None => panic!("Expected RParen, found None"),
            }
        }
        panic!("Expected RParen, found EOF");
    }
}

// Constructor
impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Parser {
        Parser { tokens }
    }
}
