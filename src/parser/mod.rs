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
}


impl Parser {
    pub fn parse(input: String, verbose: bool) -> Statement {
        let mut parser: Parser = Parser {
            error_handler: ErrorHandler::new(verbose),
            lexer: Lexer::new(input),
        }; 
        let statement: Statement = parser.parse_statement();
        //println!("{:#?}", statement);
        parser.error_handler.print_errors(); 
        return statement;
    }


    fn next_token(&mut self) -> Token {
        let token: Result<Token, &'static str> = self.lexer.next_token();
        match token {
            Ok(t) => return t,
            Err(_) => {
                self.error_handler.add(Error::Error("Unexpected end of file.".to_string()));
                panic!();
            },
        };
    }


    fn peak(&mut self) -> Token {
        let token: Result<Token, &'static str> = self.lexer.peak();
        match token {
            Ok(t) => return t,
            Err(_) => {
                self.error_handler.add(Error::Error("Unexpected end of file.".to_string())); 
                panic!();
            },
        };
    }

    fn get_original_start(&mut self) -> usize {
        return self.lexer.get_original_start();
    }

    fn get_original(&mut self, start: usize) -> String {
        return self.lexer.get_original(start);
    }


    fn parse_type(&mut self, token_type: TokenType, original_start: usize) -> Token {
        let token: Token = self.peak();
        if token.get_type() == token_type {
            return self.next_token(); 
        } else {
            let code: String = self.get_original(original_start);
            
            self.error_handler.add(Error::SyntaxError(SyntaxError {
                level: ErrorLevel::Error,
                message: format!("Expected {:?}.", token_type).to_string(),
                code: code,
                line: token.get_line(),
                offset: token.get_offset(),
            }));
            return self.create_dummy(token_type); 
        }
    }


    fn is_tokentype(&mut self, token_type: TokenType) -> bool {
        let token: Token = self.peak();
        return token.get_type() == token_type;
    }


    fn create_dummy(&mut self, token_type: TokenType) -> Token {
       return Token::new(token_type, "Dummy".to_string(), 0, 0);
    }
}

