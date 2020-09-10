#![allow(dead_code)]

mod environment;
mod function_env;
mod type_statements;
mod type_expressions;
mod type_literals;
mod type_operations;
pub mod r#type;


pub use super::span::Span;

pub use r#type::{
    Type,
};

pub use super::error::{
    ErrorLevel,
    Error,
    ErrorHandler,
    TypeError,
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

        //println!("{:#?}", type_checker);
        
        type_checker.error_handler.print_errors();

        return false; 
    }

    fn create_error(&mut self, message: String) -> () {
        let error: Error = Error::Error(message.clone());

        self.error_handler.add(error);
    }

    fn create_type_error(&mut self, level: ErrorLevel, message: String, code: Span<String>, line: usize, offset: usize) -> () {
        let error: Error = Error::TypeError(TypeError {
            level: level,
            message: message.clone(),
            code: code,
            line: line,
            offset: offset,
        });

        self.error_handler.add(error);
    }


    fn add_function(&mut self, identifier: Span<String>, env_id: usize, original: Span<String>) -> () {

        match self.search_function(identifier.clone()) {
            Ok(_) => {
                self.create_type_error(ErrorLevel::Error,
                                       format!("Function {:#?} is already decleard", identifier.get_fragment()).to_string(),
                                       original,
                                       identifier.get_line(),
                                       identifier.get_offset());
            },
            Err(_) => {
                self.get_environment().add_function(identifier.clone(), env_id);
            },
        };
    }

    fn search_function(&mut self, identifier: Span<String>) -> Result<Function, String> {
        let mut env_id_r: Option<usize> = self.current_env_id;
        loop {
            match env_id_r {
                Some(env_id) => {
                    match self.environments[env_id].lookup_function_id(identifier.get_fragment(), self.current_body_id) {
                        Ok(id) => {
                            return Ok(self.environments[id].function.clone());
                        },
                        Err(_) => env_id_r = self.environments[env_id].previus_id,
                    };   
                },
                None => break,
            };
        }
        match self.environment.lookup_function(identifier.get_fragment()) {
            Ok(id) => {
                return Ok(self.environments[id].function.clone());
            },
            Err(msg) => return Err(msg),
        };   
    }

    fn lookup_function(&mut self, identifier: Span<String>, original: Span<String>) -> Function {
        match self.search_function(identifier.clone()) {
            Ok(function) => return function,
            Err(msg) => {
                self.create_type_error(ErrorLevel::Error, msg, original, identifier.get_line(), identifier.get_offset());
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

    fn lookup_variable(&mut self, identifier: Span<String>, original: Span<String>) -> Type {
        match self.current_env_id {
            Some(id) => {
                match self.environments[id].lookup_variable(identifier.get_fragment(), self.current_body_id) {
                    Ok(val) => return val,
                    Err(msg) => {
                        self.create_type_error(ErrorLevel::Error, msg, original, identifier.get_line(), identifier.get_offset());
                        return Type::Any;
                    },
                };
            },
            None => {
                match self.environment.lookup_variable(identifier.get_fragment()) {
                    Ok(val) => return val,
                    Err(msg) => {
                        self.create_type_error(ErrorLevel::Error, msg, original, identifier.get_line(), identifier.get_offset());
                        return Type::Any;
                    },
                }
            },
        };
    }

    fn get_environment(&mut self) -> &mut Environment {
        return match self.current_env_id {
            Some(env_id) => &mut self.environments[env_id].environments[self.current_body_id],
            None => &mut self.environment,
        };
    }

    fn new_function_env(&mut self, function: Function) -> () {
        let previus_env_id: Option<usize> = self.current_env_id;
        let current_env_id: usize = self.environments.len();
        
        self.current_body_id = 0;
        self.environments.push(FunctionEnv::new(current_env_id, previus_env_id, function.clone()));
        self.add_function(function.identifier.clone(), current_env_id, function.original);
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

    fn check_if_all_bodies_return(&mut self) -> () {
        match self.current_env_id {
            Some(id) => {
                if !self.environments[id].check_if_all_bodies_return() {
                    let function: Function = self.get_function();
                    self.create_type_error(ErrorLevel::Error, 
                                           format!("Function {:#?} dosen't return value in every branch", function.identifier.get_fragment()),
                                           function.original,
                                           function.identifier.get_line(),
                                           function.identifier.get_offset());
                }
            },
            None => panic!("Fatal error in type checker!!!"),
        }; 
    }
}

