#![allow(dead_code)]

pub use super::{
    Interpreter,
    Statement,
    expressions::Expression,
//    TypeDecleration,
    Span,
    Literal,
};


pub use super::statement::{
    Function,
    While,
    If,
    Let,
    Assignment,
    Return,
    Body,
};


impl Interpreter {
    pub fn interpret_statement(&mut self, stmt: Statement) -> () {
        match stmt {
            Statement::Function(_) => (),
            Statement::While(_) => (),
            Statement::If(_) => (),
            Statement::Let(_) => (),
            Statement::Assignment(_) => (),
            Statement::Return(_) => (),
            Statement::Body(_) => (),
            Statement::Expression(_) => (),
            _ => panic!("Fatal Interpreter Error"),
        }; 
    }
}

