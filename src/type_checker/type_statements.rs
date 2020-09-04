#![allow(dead_code)]


pub use super::{
    TypeChecker,
    Statement,
    expressions::Expression,
    TypeDecleration,
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


impl TypeChecker {
    pub(super) fn check_statement(&mut self, statement: Statement) -> () {
        match statement {
            Statement::Function(function) => self.check_function(*function),
            Statement::While(r#while) => self.check_while(*r#while),
            Statement::If(r#if) => self.check_if(*r#if),
            Statement::Let(r#let) => self.check_let(r#let),
            Statement::Assignment(assignment) => self.check_assignment(assignment),
            Statement::Return(r#return) => self.check_return(r#return),
            Statement::Body(body) => self.check_body(*body),
            Statement::Expression(expression) => self.check_expression(expression),
            _ => panic!("Not implemented!"),
        };
    }

    fn check_function(&mut self, function: Function) -> () {
        // TODO: Also store the parameters types and the return type?
        self.add_function(function.identifier.get_fragment(),
                          function.return_type.r#type.get_fragment());
        self.check_body(function.body);
    }

    fn check_while(&mut self, while_statement: While) -> () {
        let condition_type: String = self.get_expression_type(while_statement.condition);
        if condition_type != "bool" {
            self.create_error("type error: in while statement.".to_string());
        }

        self.check_body(while_statement.body);
    }

    fn check_if(&mut self, if_statement: If) -> () {
        let condition_type: String = self.get_expression_type(if_statement.condition);
        if condition_type != "bool" {
            self.create_error("type error: in if statement.".to_string());
        }

        self.check_body(if_statement.if_body);
        match if_statement.else_body {
            Some(body) => self.check_body(body),
            None => (),
        };
    }

    fn check_let(&mut self, let_statement: Let) -> () {
        let variable_type: String = let_statement.type_dec.r#type.get_fragment();
        self.add_variable(let_statement.identifier.get_fragment(), variable_type.clone());
        
        let expression_type: String = self.get_expression_type(let_statement.value);
        if variable_type != expression_type {
            self.create_error("type error: in let statement.".to_string());
        }
    }
    
    fn check_assignment(&mut self, assignment: Assignment) -> () {
        let variable_type: String = self.lookup_variable(assignment.identifier);
        
        let expression_type: String = self.get_expression_type(assignment.value);
        if variable_type != expression_type {
            self.create_error("type error: in assignment statement.".to_string());
        }
    }

    fn check_return(&mut self, return_statement: Return) -> () {
        let expression_type: String = self.get_expression_type(return_statement.value);
        // TODO: Check that it mathes the function return type.
    }

    fn check_body(&mut self, body: Body) -> () {
        for statement in body.body.iter() {
            self.check_statement(statement.clone());
        } 
    }

    fn check_expression(&mut self, expression: Expression) -> () {
        let expression_type: String = self.get_expression_type(expression);
        if expression_type == "" {
            // TODO: Create a warrning.
        }
    }

}

