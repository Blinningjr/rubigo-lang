#![allow(dead_code)]

pub use super::{
    Interpreter,
    Statement,
    expressions::Expression,
//    TypeDecleration,
    
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
    pub(super) fn interpret_statement(&mut self, statement: Statement) -> () {
        match statement {
            Statement::Function(function) => self.interpret_function(*function),
            Statement::While(r#while) => self.interpret_while(*r#while),
            Statement::If(r#if) => self.interpret_if(*r#if),
            Statement::Let(r#let) => self.interpret_let(r#let),
            Statement::Assignment(assignment) => self.interpret_assignment(assignment),
            Statement::Return(r#return) => self.interpret_return(r#return),
            Statement::Body(body) => self.interpret_body(*body, true),
            Statement::Expression(expression) => self.interpret_expression_statement(expression),
            _ => panic!("Not implemented!"),
        };
    }

    fn interpret_function(&mut self, function: Function) -> () {
    }

    fn interpret_while(&mut self, while_statement: While) -> () {
    }

    fn interpret_if(&mut self, if_statement: If) -> () {
    }

    fn interpret_let(&mut self, let_statement: Let) -> () {
    }
    
    fn interpret_assignment(&mut self, assignment: Assignment) -> () {
    }

    fn interpret_return(&mut self, return_statement: Return) -> () {
    }

    fn interpret_body(&mut self, body: Body, create_env: bool) -> () {
    }

    fn interpret_expression_statement(&mut self, expression: Expression) -> () {
    }
}

