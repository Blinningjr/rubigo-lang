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
    environment: Environment,

    current_env_id: Option<usize>,
    current_body_id: usize,
}


impl TypeChecker {
    pub fn type_check(ast: Statement) -> bool {
        let mut type_checker: TypeChecker = TypeChecker{
            error_handler: ErrorHandler::new(true),
            environments: vec!(),
            environment: Environment::new(0, None),
            
            current_env_id: None,
            current_body_id: 0,
        };

        type_checker.check_statement(ast);
        
        type_checker.error_handler.print_errors();

        return false; 
    }

    fn create_error(&mut self, message: String) -> () {
        let error: Error = Error::TypeError(message.clone());

        self.error_handler.add(error);
    }

    fn add_function(&mut self, identifier: Span<String>, env_id: usize) -> () {
        if !self.get_environment().add_function(identifier.clone(), env_id) {
            self.create_error(format!("Function {:#?} is already decleard", identifier.get_fragment()).to_string());
        }
    }

    fn lookup_function(&mut self, identifier: Span<String>) -> Function {
        let mut env_id_r: Option<usize> = self.current_env_id;
        loop {
            match env_id_r {
                Some(env_id) => {
                    match self.environments[env_id].lookup_function_id(identifier.get_fragment(), self.current_body_id) {
                        Ok(id) => {
                            return self.environments[id].function.clone();
                        },
                        Err(_) => env_id_r = self.environments[env_id].previus_id,
                    };   
                },
                None => break,
            };
        }
        match self.environment.lookup_function(identifier.get_fragment()) {
            Ok(id) => {
                return self.environments[id].function.clone();
            },
            Err(_) => {
                self.create_error("function not decleared".to_string());
                return Function::create_dummy();
            },
        };   

    }
     
    fn add_variable(&mut self, identifier: Span<String>, r#type: Span<String>) -> () {
        match self.current_env_id {
            Some(id) => self.environments[id].add_variable(identifier, r#type, self.current_body_id),
            None => self.environment.add_variable(identifier, r#type),
        };
    }

    fn lookup_variable(&mut self, identifier: Span<String>) -> String {
        match self.current_env_id {
            Some(id) => {
                match self.environments[id].lookup_variable(identifier.get_fragment(), self.current_body_id) {
                    Ok(val) => return val,
                    Err(msg) => {
                        self.create_error(msg);
                        return "".to_string();
                    },
                };
            },
            None => {
                match self.environment.lookup_variable(identifier.get_fragment()) {
                    Ok(val) => return val,
                    Err(msg) => {
                        self.create_error(msg);
                        return "".to_string();
                    },
                }
            },
        };
    }

    fn get_environment(&mut self) -> &mut Environment {
        return match self.current_env_id {
            Some(env_id) => &mut self.environments[env_id].environments[0],
            None => &mut self.environment,
        };
    }

    fn new_function_env(&mut self, function: Function) -> () {
        let previus_env_id: Option<usize> = self.current_env_id;
        let current_env_id: usize = self.environments.len();
        
        self.current_body_id = 0;
        self.environments.push(FunctionEnv::new(current_env_id, previus_env_id, function.clone()));
        self.get_environment().add_function(function.identifier.clone(), current_env_id);
        self.current_env_id = Option::Some(current_env_id);
    }

    fn get_function(&mut self) -> Function {
        match self.current_env_id {
            Some(id) => return self.environments[id].function.clone(),
            None => {
                self.create_error("Can't return outside function environment".to_string());
                return Function::create_dummy();
            },
        };
    }

    fn create_body(&mut self) -> () {
        match self.current_env_id {
            Some(id) => self.current_body_id = self.environments[id].create_env(self.current_body_id),
            None => panic!("Can't create body in module"),
        };
    }
}

