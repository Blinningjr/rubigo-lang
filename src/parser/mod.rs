#![allow(dead_code)]

pub mod literal;
pub mod operations;
pub mod type_decleration;
pub mod expressions;
pub mod statement;

pub use super::span::Span;

pub use super::error::{
    ErrorLevel,
    Error,
    SyntaxError,
    ErrorHandler,
};

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
    error_handler: ErrorHandler,
    lexer: Lexer,
    last_token: Option<Token>,
}


impl Parser {
    pub fn parse(input: String, verbose: bool) -> Statement {
        let mut parser: Parser = Parser {
            error_handler: ErrorHandler::new(verbose),
            lexer: Lexer::new(input),
            last_token: None,
        }; 
        let statement: Statement = parser.parse_statement();
        parser.error_handler.print_errors();
        return statement;
    }


    fn next_token(&mut self) -> Token {
        let token: Result<Token, &'static str> = self.lexer.next_token();
        match token {
            Ok(t) => {
                self.last_token = Some(t.clone());
                return t;
            },
            Err(_) => {
                self.error_handler.add(Error::Error("Unexpected end of file.".to_string()));
                self.error_handler.print_errors();
                panic!();
            },
        };
    }


    fn peak(&mut self) -> Token {
        let token: Result<Token, &'static str> = self.lexer.peak();
        match token {
            Ok(t) => return t,
            Err(_) => {
                return Token::new(
                    TokenType::EOF,
                    "".to_string(),
                    0,
                    0);
            },
        };
    }


    fn get_original_start(&mut self) -> usize {
        return self.lexer.get_original_start();
    }


    fn get_original(&mut self, start: usize) -> Span<String> {
        return self.lexer.get_original(start);
    }


    fn parse_type(&mut self, token_type: TokenType) -> Token {
        let token: Token = self.peak();
        if token.get_type() == token_type {
            return self.next_token(); 
        } else {
            self.create_error(ErrorLevel::Error,
                              format!("Expected '{}'", token_type.revert()).to_string());
            
            return self.create_dummy_token(token_type); 
        }
    }


    fn is_tokentype(&mut self, token_type: TokenType) -> bool {
        let token: Token = self.peak();
        return token.get_type() == token_type;
    }


    fn create_dummy_token(&mut self, token_type: TokenType) -> Token {
       return Token::new(token_type, "Dummy".to_string(), 0, 0);
    }


    fn create_error(&mut self, error_level: ErrorLevel,  message: String) -> () {
        let code: String = self.get_line();
        let token: &Token = self.last_token.as_ref().unwrap(); 
        
        self.error_handler.add(Error::SyntaxError(SyntaxError {
            level: error_level,
            message: message,
            code: code,
            line: token.get_line(),
            offset: token.get_end_offset(),
        }));
    }


    fn get_line(& self) -> String {
        return self.lexer.get_line();
    }


    fn create_span<T: Clone>(& self, value: T, token: & Token) -> Span<T> {
        return Span::new(value, token.get_line(), token.get_offset());
    }


    fn create_dummy_span<T: Clone>(value: T) -> Span<T> {
        return Span::new(value, 0, 0);
    }
}

