use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        ("define", TokenType::Define),
        ("lambda", TokenType::Lambda),
        ("print", TokenType::Print),
        ("if", TokenType::If),
        ("true", TokenType::True),
        ("false", TokenType::False),
    ]);
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub _type: TokenType,
    pub span: (usize, usize),
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    True,
    False,
    Integer,
    String,
    LParen,
    RParen,
    Plus,
    Minus,

    Gt,
    Lt,
    Eq,

    Identifier,
    Define,
    Lambda,
    Print,
    If,

    // Lexer produces an Error Token
    // for resiliense. We don't want to
    // stop the whole program for a single error.
    Error,
}

pub fn tokenize(string: &str) -> Vec<Token> {
    let chars: Vec<char> = string.chars().collect();
    let mut current = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while current < chars.len() {
        let span_start = current;
        let token_type = match chars[current] {
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            '>' => TokenType::Gt,
            '<' => TokenType::Lt,
            '=' => TokenType::Eq,
            '0'..='9' => {
                while current < chars.len() && chars[current].is_numeric() {
                    current += 1;
                }
                current -= 1; // Decrement current to point to the last digit
                TokenType::Integer
            }
            '"' => {
                current += 1;
                while current < chars.len() && chars[current] != '"' {
                    current += 1;
                }

                if current >= chars.len() {
                    println!("Error: Missing \" for string literal");
                }

                TokenType::String
            }
            ' ' | '\n' | '\t' => {
                current += 1;
                continue;
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                // @TODO: That's lazy - Improve performance:
                let mut content = String::new();
                while current < chars.len()
                    && (chars[current].is_alphanumeric() || chars[current] == '_')
                {
                    content.push(chars[current]);
                    current += 1;
                }
                current -= 1;

                match KEYWORDS.get(content.to_string().as_str()) {
                    Some(token) => *token,
                    None => TokenType::Identifier,
                }
            }
            _ => TokenType::Error,
        };

        tokens.push(Token {
            _type: token_type,
            span: (span_start, current),
        });

        current += 1;
    }

    tokens
}
