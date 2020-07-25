#![allow(dead_code)]

pub mod literal;
pub mod operations;
pub mod type_decleration;
pub mod expressions;
pub mod statement;

pub use super::span::Span;

use super::lexer::{
    Lexer,
    Token,
    TokenType,
};

pub use literal::{
    Literal,
};

pub use operations::{
    UnOperator,
    UnOp,
    BinOperator,
    BinOp,
};

pub use type_decleration::{
    TypeDecleration,
};

pub use expressions::{
    Expression,
};

pub use statement::{
    Statement,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    lexer: Lexer,
    tokens: Vec<Token>,
}


impl Parser {
    pub fn parse(input: String) -> Statement {
        let mut parser: Parser = Parser {
            lexer: Lexer::new(input),
            tokens: Vec::new(),
        }; 
        return parser.parse_statement();
    }


    fn next_token(&mut self) -> Token {
        let token: Token = self.lexer.next_token().unwrap();
        self.tokens.push(token.clone());
        return token; 
    }


    fn peak(&mut self) -> Token {
        let token: Token = self.lexer.peak().unwrap();
        return token; 
    }


    fn parse_type(&mut self, token_type: TokenType) -> Token {
        let token: Token = self.next_token();
        if token.get_type() == token_type {
            return token; 
        } else {
            panic!("Expected token type {:?} got {:#?}", token_type, token);
        }
    }


    fn empty_tokens(&mut self) -> () {
        self.tokens = Vec::new();
    }
    

    fn is_tokentype(&mut self, token_type: TokenType) -> bool {
        let token: Token = self.peak();
        return token.get_type() == token_type;
    }
}

