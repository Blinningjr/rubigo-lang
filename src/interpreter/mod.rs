#![allow(dead_code)]

mod operation_interp;
mod expression_interp;
mod statement_interp;

pub use super::span::Span;

pub use super::error::{
//    ErrorLevel,
//    Error,
    ErrorHandler,
};

pub use super::parser::{
    statement,
    Statement,
    expressions,
    operations,
    Literal,
    TypeDecleration,
};


pub use super::type_checker::{
    Modual,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Interpreter {
    pub error_handler: ErrorHandler,
    pub modual: Modual,
}


impl Interpreter {
    pub fn interpret(ast: Statement, modual: Modual) -> String {
        let mut interpreter: Interpreter = Interpreter{
            error_handler: ErrorHandler::new(true),
            modual: modual,
        };

        return "".to_string();//interpreter.interpret_statement(ast).to_string();
    }
}

