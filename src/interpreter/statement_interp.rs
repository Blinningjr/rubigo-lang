#![allow(dead_code)]

pub use super::{
    Interpreter,
    Statement,
    expressions::Expression,
//    TypeDecleration,

};

use crate::parser::literal::Literal;

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
        return;
    }

    fn interpret_while(&mut self, while_statement: While) -> () {
        let mut condition: Literal = self.interpret_expression(while_statement.condition.clone());
        while self.get_bool(condition) {
            self.interpret_body(while_statement.body.clone(), true);
            condition = self.interpret_expression(while_statement.condition.clone());
        }
        return;
    }

    fn interpret_if(&mut self, if_statement: If) -> () {
        let condition: Literal = self.interpret_expression(if_statement.condition);
        if self.get_bool(condition) {
            self.interpret_body(if_statement.if_body, true);
        } else {
            match if_statement.else_body {
                Some(body) => self.interpret_body(body, true),
                None => (),
            };
        }
    }

    fn interpret_let(&mut self, let_statement: Let) -> () {
        //TODO
    }
    
    fn interpret_assignment(&mut self, assignment: Assignment) -> () {
        //TODO
    }

    fn interpret_return(&mut self, return_statement: Return) -> () {
        //TODO
    }

    fn interpret_body(&mut self, body: Body, create_env: bool) -> () {
       let statements: Vec<Statement> = body.body;
       for stmt in statements {
            self.interpret_statement(stmt);
       } 
    }

    fn interpret_expression_statement(&mut self, expression: Expression) -> () {
        match expression {
            Expression::FunctionCall(f_call) => self.interpret_function_call(*f_call),
            _ => panic!("Fatal Interpreter error"),
        };
    }
}

