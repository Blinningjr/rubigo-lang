#![allow(dead_code)]

pub use super::{
    Interpreter,
    Statement,
    expressions::Expression,
//    TypeDecleration,
    Span,
    Literal,
    Value,
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
            Statement::Function(func) => self.store_function(*func),
            Statement::While(_) => (),
            Statement::If(_) => (),
            Statement::Let(let_stmt) => self.interpret_let(let_stmt),
            Statement::Assignment(_) => (),
            Statement::Return(_) => (),
            Statement::Body(_) => (),
            Statement::Expression(_) => (),
            _ => panic!("Fatal Interpreter Error"),
        }; 
    }

    pub fn interpret_let(&mut self, let_stmt: Let) -> () {
        let name: Span<String> = let_stmt.identifier;
        let value: Value = self.interpret_expression(let_stmt.value);

        self.store_variable(name.get_fragment(), value);
    }
    
}

