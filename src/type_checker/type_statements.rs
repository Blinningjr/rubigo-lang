#![allow(dead_code)]


pub use super::{
    TypeChecker,
    Statement,
    expressions::Expression,
    TypeDecleration,
};

pub use super::r#type::{
    Type,
    compare_types,
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
        self.check_if_unreachable_code();
        
        match statement {
            Statement::Function(function) => self.check_function(*function),
            Statement::While(r#while) => self.check_while(*r#while),
            Statement::If(r#if) => self.check_if(*r#if),
            Statement::Let(r#let) => self.check_let(r#let),
            Statement::Assignment(assignment) => self.check_assignment(assignment),
            Statement::Return(r#return) => self.check_return(r#return),
            Statement::Body(body) => self.check_body(*body, true),
            Statement::Expression(expression) => self.check_expression(expression),
            _ => panic!("Not implemented!"),
        };
    }

    fn check_function(&mut self, function: Function) -> () {
        let current_id = self.current_env_id;
        let current_body_id = self.current_body_id;

        self.new_function_env(function.clone());
        self.check_body(function.body, false);

        self.check_if_all_bodies_return();

        self.current_env_id = current_id;
        self.current_body_id = current_body_id;
    }

    fn check_while(&mut self, while_statement: While) -> () {
        let condition_type: Type = self.get_expression_type(while_statement.condition);
        if !compare_types(&condition_type, &Type::Custom("bool".to_string())) {
            self.create_error("type error: in while statement.".to_string());
        }

        self.check_body(while_statement.body, true);
    }

    fn check_if(&mut self, if_statement: If) -> () {
        let condition_type: Type = self.get_expression_type(if_statement.condition);
        if !compare_types(&condition_type, &Type::Custom("bool".to_string())) {
            self.create_error("type error: in if statement.".to_string());
        }

        self.check_body(if_statement.if_body, true);

        match self.current_env_id {
            Some(id) => {
                let env_id: usize = self.environments[id].environments.len() - 1;
                self.environments[id].environments[env_id].if_body = true;
            },
            None => panic!("Fatal error in type checker!!!"),
        };

        match if_statement.else_body {
            Some(body) => self.check_body(body, true),
            None => (),
        };
    }

    fn check_let(&mut self, let_statement: Let) -> () {
        let variable_type: Type = Type::Custom(let_statement.type_dec.r#type.get_fragment());
        self.add_variable(let_statement.identifier, let_statement.type_dec.r#type);
        
        let expression_type: Type = self.get_expression_type(let_statement.value); 
        if !compare_types(&variable_type, &expression_type) {
            self.create_error("type error: in let statement.".to_string());
        }
    }
    
    fn check_assignment(&mut self, assignment: Assignment) -> () {
        let variable_type: Type = self.lookup_variable(assignment.identifier);
        
        let expression_type: Type = self.get_expression_type(assignment.value);
        if !compare_types(&variable_type, &expression_type) {
            self.create_error("type error: in assignment statement.".to_string());
        }
    }

    fn check_return(&mut self, return_statement: Return) -> () {
        self.get_environment().returns_value = true;
        
        let expression_type: Type = self.get_expression_type(return_statement.value);
        let return_type: Type = Type::Custom(self.get_function().return_type.r#type.get_fragment());

        if !compare_types(&expression_type, &return_type) {
            self.create_error("type error: in return statement.".to_string());
        }
    }

    fn check_body(&mut self, body: Body, create_env: bool) -> () {
        let current_body_id: usize = self.current_body_id;
        if create_env {
            self.create_body();
        }

        for statement in body.body.iter() {
            self.check_statement(statement.clone());
        } 
        self.current_body_id = current_body_id;
    }

    fn check_expression(&mut self, expression: Expression) -> () {
        let expression_type: Type = self.get_expression_type(expression);
        if !compare_types(&expression_type, &Type::Custom("".to_string())) {
            // TODO: Create a warrning.
        }
    }

    fn check_if_unreachable_code(&mut self) -> () {
        if self.get_environment().returns_value {
            self.create_error("Warrning: Unreachable code".to_string());
        }
    }

}

