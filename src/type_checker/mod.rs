#![allow(dead_code)]

mod environment;
mod statements;
mod expressions;

pub use super::error::{
    ErrorLevel,
    Error,
    ErrorHandler,
};

pub use super::parser::{
    statement,
    Expression,
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

        match ast {
            Statement::Let(let_statement) => type_checker.check_let(let_statement),
            _ => (),
        };
        
        type_checker.error_handler.print_errors();

        return false; 
    }

    fn create_error(&mut self, message: String) -> () {
        let error: Error = Error::TypeError(message.clone());

        self.error_handler.add(error);
    }

    fn add_variable(&mut self, identifier: String, r#type: String) -> () {
        if !self.environment.add_variable(identifier.clone(), r#type) {
            self.create_error(format!("{:#?} is already decleard", identifier).to_string());
        }
    }
}

