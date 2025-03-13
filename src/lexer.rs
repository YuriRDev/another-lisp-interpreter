use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(i64),
    String(String),
    Symbol(String),
    LParen,
    RParen,
}

pub fn tokenize(string: &str) -> VecDeque<Token> {
    let chars: Vec<char> = string.chars().collect();
    let mut current = 0;
    let mut tokens: VecDeque<Token> = VecDeque::new();

    while current < chars.len() {
        match chars[current] {
            '(' => {
                tokens.push_back(Token::LParen);
                current += 1;
            }
            ')' => {
                tokens.push_back(Token::RParen);
                current += 1;
            }
            '0'..='9' => {
                let mut number = String::new();
                while current < chars.len() && chars[current].is_digit(10) {
                    number.push(chars[current]);
                    current += 1;
                }
                tokens.push_back(Token::Integer(number.parse::<i64>().unwrap()));
            }
            '"' => {
                let mut string = String::new();
                current += 1;
                while current < chars.len() && chars[current] != '"' {
                    string.push(chars[current]);
                    current += 1;
                }
                tokens.push_back(Token::String(string));
                current += 1;
            }
            ';' => {
                while current < chars.len() && chars[current] != '\n' {
                    current += 1;
                }
            }
            ' ' => {
                current += 1;
            }
            _ => {
                let mut symbol = String::new();
                while current < chars.len() && !chars[current].is_whitespace() {
                    symbol.push(chars[current]);
                    current += 1;
                }
                tokens.push_back(Token::Symbol(symbol))
            }
        }
    }

    tokens
}
