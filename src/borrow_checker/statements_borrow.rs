#![allow(dead_code)]

pub use super::{
    BorrowChecker,
    expressions::Expression,
    TypeDecleration,
    Span,
    ErrorLevel,

    BorrowFunction,
    BorrowVariable,
};


pub use super::statement::{
    Statement,
    Function,
    Parameter,
    While,
    If,
    Let,
    Assignment,
    Return,
    Body,
};


impl BorrowChecker {
    pub(super) fn check_statement(&mut self, statement: Statement) -> () { 
        match statement {
            Statement::Function(function) => self.check_function(*function),
            Statement::While(r#while) => self.check_while(*r#while),
            Statement::If(r#if) => self.check_if(*r#if),
            Statement::Let(r#let) => self.check_let(r#let),
            Statement::Assignment(assignment) => self.check_assignment(assignment),
            Statement::Return(r#return) => self.check_return(r#return),
            Statement::Body(body) => self.check_body(*body, true),
            Statement::Expression(expression) => {self.check_expression(expression);},
            _ => panic!("Not implemented!"),
        };
    }

    pub(super) fn check_function(&mut self, function: Function) -> () {
    }

    fn check_while(&mut self, while_stmt: While) -> () {
        self.check_expression(while_stmt.condition);
        self.check_body(while_stmt.body, true);
    }

    fn check_if(&mut self, if_stmt: If) -> () {
        self.check_expression(if_stmt.condition);
        self.check_body(if_stmt.if_body, true);
        if let Some(body) = if_stmt.else_body {
            self.check_body(body, true);
        }
    }

    fn check_let(&mut self, let_stmt: Let) -> () {
        let value = self.check_expression(let_stmt.value);
        let mutable = let_stmt.mutable != None;
        let pointer = self.store_value(value, mutable);
        self.add_variable(
                let_stmt.original.clone(),
                let_stmt.identifier,
                mutable,
                pointer,
            );
    }
    
    fn check_assignment(&mut self, assignment: Assignment) -> () {
        let value = self.check_expression(assignment.value);
        self.update_variable(
            assignment.identifier.fragment,
            assignment.derefrenced != None,
            value
            );
        // self.set_variable(); \\TODO: update varaible in mem. remove old pointer?
    }

    fn check_return(&mut self, return_stmt: Return) -> () {
        let value = self.check_expression(return_stmt.value);
    }

    fn check_body(&mut self, body: Body, create_env: bool) -> () {
        let current_env: usize = self.current_env;

        if create_env {
            self.create_body();
        }

        for statement in body.body.iter() {
            self.check_statement(statement.clone());
        } 

        // TODO: Remove all variables added in this scope. This is not nedded i think?

        self.current_env = current_env;
        if self.current_func == None {
            self.current_mod_env = current_env;
        }
    }
}

