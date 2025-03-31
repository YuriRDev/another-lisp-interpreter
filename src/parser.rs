use crate::lexer::{Token, TokenType};
#[derive(Debug)]
pub enum ASTType {
    LTop,
    GTop,
    EQop,

    Plus,
    Minus,

    If,
    Define(String),

    Number(i64),
    String(String),
    Boolean(bool),
    Print,
}

#[derive(Debug)]
pub struct AST {
    pub _type: ASTType,
    pub children: Vec<AST>,
}

impl AST {
    fn new(t: ASTType) -> AST {
        AST {
            _type: t,
            children: Vec::new(),
        }
    }

    // @TODO: Change this method name, is kinda bad lol.
    fn new_populate(t: ASTType, children: Vec<AST>) -> AST {
        AST { _type: t, children }
    }
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
                AST::new(ASTType::Boolean(true))
            }
            TokenType::False => {
                self.consume(TokenType::False);
                AST::new(ASTType::Boolean(false))
            }
            TokenType::If => self.parse_if(),
            TokenType::Print => self.parse_print(),

            TokenType::Plus => self.parse_plus(),
            TokenType::Minus => self.parse_minus(),

            TokenType::Lt => self.parse_lt(),
            TokenType::Gt => self.parse_gt(),
            TokenType::Eq => self.parse_eq(),

            TokenType::Integer => {
                let (start, end) = self.tokens[self.current].span;
                self.consume(TokenType::Integer);

                let atom = &self.src[start..end + 1];
                let value: i64 = atom.parse::<i64>().unwrap();

                AST::new(ASTType::Number(value))
            }

            TokenType::String => {
                let (start, end) = self.tokens[self.current].span;
                self.consume(TokenType::String);

                let atom = &self.src[start..end + 1];

                AST::new(ASTType::String(atom.to_string()))
            }

            TokenType::LParen => {
                self.consume(TokenType::LParen);
                let expr = self.parse_expr();
                self.consume(TokenType::RParen);
                expr
            }
            _ => todo!("Missing tokens at parse_expr"),
        }
    }

    fn parse_plus(&mut self) -> AST {
        self.consume(TokenType::Plus);
        let children = self.consume_list();

        AST::new_populate(ASTType::Plus, children)
    }

    fn parse_lt(&mut self) -> AST {
        self.consume(TokenType::Lt);
        let children = self.consume_list();
        if children.len() != 2 {
            panic!(
                "Error, expected 2 args at `lt`, received {}",
                children.len()
            )
        }

        AST::new_populate(ASTType::LTop, children)
    }

    fn parse_eq(&mut self) -> AST {
        self.consume(TokenType::Eq);
        let children = self.consume_list();
        if children.len() != 2 {
            panic!(
                "Error, expected 2 args at `eq`, received {}",
                children.len()
            )
        }

        AST::new_populate(ASTType::EQop, children)
    }

    fn parse_gt(&mut self) -> AST {
        self.consume(TokenType::Gt);
        let children = self.consume_list();
        if children.len() != 2 {
            panic!(
                "Error, expected 2 args at `gt`, received {}",
                children.len()
            )
        }

        AST::new_populate(ASTType::GTop, children)
    }

    fn parse_minus(&mut self) -> AST {
        self.consume(TokenType::Minus);
        let children = self.consume_list();

        AST::new_populate(ASTType::Minus, children)
    }

    fn parse_if(&mut self) -> AST {
        self.consume(TokenType::If);
        self.consume(TokenType::LParen);
        let condition = self.parse_expr();
        self.consume(TokenType::RParen);

        self.consume(TokenType::LParen);
        let lside = self.parse_expr();
        self.consume(TokenType::RParen);

        if self.current() == TokenType::LParen {
            self.consume(TokenType::LParen);
            let rside = self.parse_expr();
            self.consume(TokenType::RParen);
            return AST::new_populate(ASTType::If, Vec::from([condition, lside, rside]));
        }

        AST::new_populate(ASTType::If, Vec::from([condition, lside]))
    }

    fn parse_print(&mut self) -> AST {
        self.consume(TokenType::Print);
        self.consume(TokenType::LParen);
        let pl = self.parse_expr();
        self.consume(TokenType::RParen);

        AST::new_populate(ASTType::Print, Vec::from([pl]))
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
