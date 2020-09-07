#![allow(dead_code)]

mod environment;
mod function_env;
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

use statement::Function;

pub use statement::Statement;

pub use environment::{
    Environment,
};

pub use function_env::FunctionEnv;


#[derive(Debug, Clone, PartialEq)]
pub struct TypeChecker {
    error_handler: ErrorHandler,
    environments: Vec<FunctionEnv>, 
    current_env_id: usize,
}


impl TypeChecker {
    pub fn type_check(ast: Statement) -> bool {
        let mut type_checker: TypeChecker = TypeChecker{
            error_handler: ErrorHandler::new(true),
            environments: vec!(FunctionEnv::new(0, 0)),
            current_env_id: 0,
        };

        type_checker.check_statement(ast);
        
        type_checker.error_handler.print_errors();

        return false; 
    }

    fn create_error(&mut self, message: String) -> () {
        let error: Error = Error::TypeError(message.clone());

        self.error_handler.add(error);
    }

    fn add_function(&mut self, identifier: Span<String>, r#type: Span<String>) -> () {
        if !self.get_environment().add_function(identifier.clone(), r#type) {
            self.create_error(format!("Function {:#?} is already decleard", identifier.get_fragment()).to_string());
        }
    }

    fn lookup_function(&mut self, identifier: Span<String>) -> String {
        return match self.get_environment().lookup_function(identifier.get_fragment()) {
            Ok(r#type) => r#type,
            Err(msg) => panic!(msg),
        };   
    }
     
    fn add_variable(&mut self, identifier: Span<String>, r#type: Span<String>) -> () {
        if !self.get_environment().add_variable(identifier.clone(), r#type) {
            self.create_error(format!("Variable {:#?} is already decleard", identifier.get_fragment()).to_string());
        }
    }

    fn lookup_variable(&mut self, identifier: Span<String>) -> String {
        return match self.get_environment().lookup_variable(identifier.get_fragment()) {
            Ok(r#type) => r#type,
            Err(msg) => panic!(msg),
        };   
    }

    fn get_environment(&mut self) -> &mut FunctionEnv {
        return &mut self.environments[self.current_env_id];
    }

    fn new_function_env(&mut self, function: Function) -> () {
        let previus_env_id: usize = self.current_env_id;
        let current_env_id: usize = self.environments.len();
        self.environments.push(FunctionEnv::new(current_env_id, previus_env_id));
        self.current_env_id = current_env_id;
        self.get_environment().function = Option::Some(function);
    }
}

