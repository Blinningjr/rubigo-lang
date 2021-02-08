#![allow(dead_code)]

pub use super::{
    BorrowChecker,
    expressions::Expression,
    TypeDecleration,
    Span,
    ErrorLevel,

    BorrowEnvironments,
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
    pub(super) fn check_statement(&mut self, envs: &mut BorrowEnvironments, statement: Statement) -> () { 
        match statement {
            Statement::Function(function) => self.add_function(*function),
            Statement::While(r#while) => self.check_while(envs, *r#while),
            Statement::If(r#if) => self.check_if(envs, *r#if),
            Statement::Let(r#let) => self.check_let(envs, r#let),
            Statement::Assignment(assignment) => self.check_assignment(envs, assignment),
            Statement::Return(r#return) => self.check_return(envs, r#return),
            Statement::Body(body) => self.check_body(envs.clone(), *body),
           // Statement::Expression(expression) => {self.check_expression(expression);}, // TODO: Check Expressions.
            _ => panic!("Not implemented!"),
        };
    }

    pub(super) fn check_function(&mut self, function: Function) -> () {
        let mut envs = self.create_envs();
        for p in function.parameters.iter() {
            envs.add_variable(p.identifier.clone(), p.mutable != None, function.original.clone());
        }
        self.check_body(envs.clone(), function.body);
    }

    fn check_while(&mut self, envs: &mut BorrowEnvironments, while_stmt: While) -> () {
//        self.check_expression(while_stmt.condition); // TODO: Check Expressions
        self.check_body(envs.clone(), while_stmt.body);
    }

    fn check_if(&mut self, envs: &mut BorrowEnvironments, if_stmt: If) -> () {
//        self.check_expression(if_stmt.condition); // TODO: Check Expressions
        self.check_body(envs.clone(), if_stmt.if_body);
        if let Some(body) = if_stmt.else_body {
            self.check_body(envs.clone(), body);
        }
    }

    fn check_let(&mut self, envs: &mut BorrowEnvironments, let_stmt: Let) -> () {
//        let value = self.check_expression(let_stmt.value); // TODO: Check Expressions.
//        let mutable = let_stmt.mutable != None;
//        let pointer = self.store_value(value, mutable);
//        self.add_variable(
//                let_stmt.original.clone(),
//                let_stmt.identifier,
//                mutable,
//                pointer,
//            );
    }
    
    fn check_assignment(&mut self, envs: &mut BorrowEnvironments, assignment: Assignment) -> () {
//        let value = self.check_expression(assignment.value); // TODO: Check Expressions.
//        self.update_variable(
//            assignment.identifier.fragment,
//            assignment.derefrenced != None,
//            value
//            );
//        // self.set_variable(); \\TODO: update varaible in mem. remove old pointer?
    }

    fn check_return(&mut self, envs: &mut BorrowEnvironments, return_stmt: Return) -> () {
//        let value = self.check_expression(return_stmt.value); // TODO: Check Expressions.
    }

    fn check_body(&mut self, mut envs: BorrowEnvironments, body: Body) -> () {
        envs.create_env();

        for statement in body.body.iter() {
            self.check_statement(&mut envs, statement.clone());
        } 

        envs.pop_env();    
    }
}

