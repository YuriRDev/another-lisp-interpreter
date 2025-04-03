use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum ArithmeticOp {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Lt,
    Gt,
    Eq,
}

#[derive(Debug, Clone)]
pub enum InputType {
    String,
    Number,
}

#[derive(Debug, Clone)]
pub enum AST {
    Binary(BinaryOp, Box<AST>, Box<AST>),

    Arithmetic(ArithmeticOp, Vec<AST>),

    If(Box<AST>, Box<AST>, Box<AST>),

    Define(String, Box<AST>),

    Identifier(String),

    Input(InputType),

    Number(i64),
    String(String),
    Boolean(bool),
    Print(Box<AST>),
}

pub struct Parser {
    tokens: Vec<Token>,
    src: String,
    current: usize,
    length: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Vec<AST> {
        let mut main_ast: Vec<AST> = Vec::new();
        while self.current < self.length {
            self.consume(TokenType::LParen);
            main_ast.push(self.parse_expr());
            self.consume(TokenType::RParen);
        }
        main_ast
    }

    fn parse_expr(&mut self) -> AST {
        match self.current() {
            TokenType::True => {
                self.consume(TokenType::True);
                AST::Boolean(true)
            }
            TokenType::False => {
                self.consume(TokenType::False);
                AST::Boolean(false)
            }
            TokenType::If => self.parse_if(),
            TokenType::Print => self.parse_print(),

            TokenType::Plus => self.parse_arithmetic_op(ArithmeticOp::Plus),
            TokenType::Minus => self.parse_arithmetic_op(ArithmeticOp::Minus),

            TokenType::Lt => self.parse_binary_op(BinaryOp::Lt),
            TokenType::Gt => self.parse_binary_op(BinaryOp::Gt),
            TokenType::Eq => self.parse_binary_op(BinaryOp::Eq),

            TokenType::Define => {
                self.consume(TokenType::Define);
                let atom = self.get_span_content();
                self.consume(TokenType::Identifier);

                self.consume(TokenType::LParen);
                let inside = self.parse_expr();
                self.consume(TokenType::RParen);

                AST::Define(atom, Box::new(inside))
            }

            TokenType::Integer => {
                let atom = self.get_span_content();
                self.consume(TokenType::Integer);

                // @TODO: Treat unwrap (possible overflow)
                AST::Number(atom.parse::<i64>().unwrap())
            }

            TokenType::String => {
                let atom = self.get_span_content();
                self.consume(TokenType::String);

                AST::String(atom)
            }

            TokenType::LParen => {
                self.consume(TokenType::LParen);
                let expr = self.parse_expr();
                self.consume(TokenType::RParen);
                expr
            }

            TokenType::RParen => {
                panic!("Unexpected RParen")
            }
            TokenType::Identifier => {
                let atom = self.get_span_content();
                self.consume(TokenType::Identifier);
                AST::Identifier(atom)
            }
            TokenType::ReadN => {
                self.consume(TokenType::ReadN);
                AST::Input(InputType::Number)
            }
            TokenType::ReadS => {
                self.consume(TokenType::ReadS);
                AST::Input(InputType::String)
            }
            _ => todo!("Missing tokens at parse_expr"),
        }
    }

    fn parse_arithmetic_op(&mut self, op: ArithmeticOp) -> AST {
        self.consume(match op {
            ArithmeticOp::Minus => TokenType::Minus,
            ArithmeticOp::Plus => TokenType::Plus,
        });
        let children = self.consume_list();

        AST::Arithmetic(op, children)
    }

    fn parse_binary_op(&mut self, op: BinaryOp) -> AST {
        self.consume(match op {
            BinaryOp::Eq => TokenType::Eq,
            BinaryOp::Lt => TokenType::Lt,
            BinaryOp::Gt => TokenType::Gt,
        });

        let children = self.consume_list();
        if children.len() != 2 {
            panic!(
                "Error, expected 2 args at binary, received {}",
                children.len()
            )
        }

        AST::Binary(
            op,
            Box::new(children[0].clone()),
            Box::new(children[1].clone()),
        )
    }

    fn parse_if(&mut self) -> AST {
        self.consume(TokenType::If);

        self.consume(TokenType::LParen);
        let condition = self.parse_expr();
        self.consume(TokenType::RParen);

        self.consume(TokenType::LParen);
        let lside = self.parse_expr();
        self.consume(TokenType::RParen);

        self.consume(TokenType::LParen);
        let rside = self.parse_expr();
        self.consume(TokenType::RParen);

        AST::If(Box::new(condition), Box::new(lside), Box::new(rside))
    }

    fn parse_print(&mut self) -> AST {
        self.consume(TokenType::Print);
        self.consume(TokenType::LParen);
        let pl = self.parse_expr();
        self.consume(TokenType::RParen);

        AST::Print(Box::new(pl))
    }

    fn consume_list(&mut self) -> Vec<AST> {
        // Everything until ) is found.
        let mut list: Vec<AST> = Vec::new();
        while self.current < self.length && self.tokens[self.current]._type != TokenType::RParen {
            list.push(self.parse_expr());
        }

        list
    }

    fn current(&self) -> TokenType {
        if self.current < self.length {
            return self.tokens[self.current]._type;
        }

        panic!("Unexpected EOF")
    }

    fn consume(&mut self, expected: TokenType) {
        if self.current >= self.length {
            panic!("Unexpected EOF. Expected {:?}", expected);
        }
        let current = &self.tokens[self.current];
        let (start, end) = current.span;
        if current._type != expected {
            panic!(
                "Expected {:?}, received {:?} at [{}:{}]",
                expected, current._type, start, end
            );
        }
        self.current += 1;
    }

    fn get_span_content(&self) -> String {
        let (start, end) = self.tokens[self.current].span;
        let atom = &self.src[start..end + 1];
        atom.to_string()
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>, src: &str) -> Parser {
        Parser {
            length: tokens.len(),
            tokens,
            src: src.to_string(),
            current: 0,
        }
    }
}
