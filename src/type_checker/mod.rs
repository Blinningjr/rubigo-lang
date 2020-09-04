#![allow(dead_code)]

mod environment;
mod type_statements;
mod type_expressions;
mod type_literals;
mod type_operations;


pub use super::span::Span;

pub use super::error::{
    ErrorLevel,
    Error,
    ErrorHandler,
};

pub use super::parser::{
    statement,
    expressions,
    operations,
    Literal,
    TypeDecleration,
};

pub use statement::Statement;

pub use environment::{
    Environment,
};


#[derive(Debug, Clone, PartialEq)]
pub struct TypeChecker {
    error_handler: ErrorHandler,
    environment: Environment, 
}


impl TypeChecker {
    pub fn type_check(ast: Statement) -> bool {
        let mut type_checker: TypeChecker = TypeChecker{
            error_handler: ErrorHandler::new(true),
            environment: Environment::new(),
        };

        type_checker.check_statement(ast);
        
        type_checker.error_handler.print_errors();

        return false; 
    }

    fn create_error(&mut self, message: String) -> () {
        let error: Error = Error::TypeError(message.clone());

        self.error_handler.add(error);
    }

    fn add_function(&mut self, identifier: String, r#type: String) -> () {
        if !self.environment.add_function(identifier.clone(), r#type) {
            self.create_error(format!("Function {:#?} is already decleard", identifier).to_string());
        }
    }

    fn lookup_function(&mut self, identifier: Span<String>) -> String {
        return match self.environment.lookup_function(identifier.get_fragment()) {
            Ok(r#type) => r#type,
            Err(msg) => panic!(msg),
        };   
    }
     
    fn add_variable(&mut self, identifier: String, r#type: String) -> () {
        if !self.environment.add_variable(identifier.clone(), r#type) {
            self.create_error(format!("Variable {:#?} is already decleard", identifier).to_string());
        }
    }

    fn lookup_variable(&mut self, identifier: Span<String>) -> String {
        return match self.environment.lookup_variable(identifier.get_fragment()) {
            Ok(r#type) => r#type,
            Err(msg) => panic!(msg),
        };   
    }
}

