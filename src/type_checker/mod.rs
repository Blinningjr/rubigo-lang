#![allow(dead_code)]

mod environment;
mod function_env;
mod modual;
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
    ModualBody,
};

pub use statement::Function;

pub use statement::Statement;

pub use environment::{
    Environment,
    Variable,
};

pub use function_env::FunctionEnv;

pub use modual::Modual;


#[derive(Debug, Clone, PartialEq)]
pub struct TypeChecker {
    pub error_handler: ErrorHandler,
    pub modual: Modual,
}


impl TypeChecker {
    pub fn type_check(ast: ModualBody, print_errors: bool) -> TypeChecker {
        let mut type_checker: TypeChecker = TypeChecker{
            error_handler: ErrorHandler::new(true),
            modual: Modual::new(ast.clone()),
        };
       
        let mut print_func: Function = Function::create_dummy();
        print_func.identifier = Span::new("print".to_string(), 0, 0);
        print_func.parameters.push((Span::new("DUMMY".to_string(), 0, 0), 
                                    TypeDecleration{borrow: false,
                                    mutable: false,
                                    r#type: Span::new(" ANY".to_string(), 0, 0)}));
        type_checker.new_function_env(print_func);
        type_checker.modual.current_env_id = None;
        type_checker.modual.current_body_id = 0;

        type_checker.check_modual_body(ast);

        //println!("{:#?}", type_checker);
        if print_errors { 
            type_checker.error_handler.print_errors();
        }

        return type_checker; 
    }

    fn check_modual_body(&mut self, mod_body: ModualBody) -> () {
        for stmt in mod_body.body.iter() {
            self.check_statement(stmt.clone());
        }
    }

    pub fn get_modual(&mut self) -> Modual {
        return self.modual.clone()
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
        match self.lookup_function(identifier.clone()) {
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

    fn lookup_function(&mut self, identifier: Span<String>) -> Result<Function, String> {
        match self.modual.current_env_id {
            Some(env_id) => {
                match self.modual.environments[env_id].lookup_function_id(identifier.get_fragment(), self.modual.current_body_id) {
                    Ok(id) => {
                        return Ok(self.modual.environments[id].function.clone());
                    },
                    Err(_) => (),
                };
            },
            None => (),
        };

        return self.modual.lookup_function_in_mod_envs(identifier);
    }

    fn add_variable(&mut self, identifier: Span<String>, type_dec: TypeDecleration) -> () {
        let variable: Variable = Variable::new(identifier, Type::Custom(type_dec.r#type.get_fragment(), type_dec.borrow, type_dec.mutable), type_dec.mutable);
        match self.modual.current_env_id {
            Some(id) => self.modual.environments[id].add_variable(variable, self.modual.current_body_id),
            None => self.modual.mod_envs[self.modual.mod_body_id].add_variable(variable),
        };
    }

    fn lookup_variable(&mut self, identifier: Span<String>, original: Span<String>) -> Variable {
        match self.modual.current_env_id {
            Some(id) => {
                match self.modual.environments[id].lookup_variable(identifier.get_fragment(), self.modual.current_body_id) {
                    Ok(val) => return val,
                    Err(_) => (),
                };
            },
            None => (),
        };

        match self.modual.lookup_variable_in_mod_envs(identifier.clone()) {
            Ok(val) => return val,
            Err(msg) => {
                self.create_type_error(ErrorLevel::Error,
                                       msg,
                                       original, identifier.get_line(), identifier.get_offset());
                return Variable::new(Span::new("".to_string(), 0, 0), Type::Any, true);
            },
        };
    }


    fn get_environment(&mut self) -> &mut Environment {
        return match self.modual.current_env_id {
            Some(env_id) => &mut self.modual.environments[env_id].environments[self.modual.current_body_id],
            None => &mut self.modual.mod_envs[self.modual.mod_body_id],
        };
    }

    fn new_function_env(&mut self, function: Function) -> () {
        let previus_env_id: Option<usize> = self.modual.current_env_id;
        let current_env_id: usize = self.modual.environments.len();
        
        self.add_function(function.identifier.clone(), current_env_id, function.original.clone());
        
        self.modual.current_body_id = 0;
        self.modual.environments.push(FunctionEnv::new(current_env_id, previus_env_id, function));
        self.modual.current_env_id = Option::Some(current_env_id);
    }

    fn get_function(&mut self) -> Function {
        match self.modual.current_env_id {
            Some(id) => return self.modual.environments[id].function.clone(),
            None => {
                self.create_error("Can't return outside function environment".to_string());
                return Function::create_dummy();
            },
        };
    }

    fn create_body(&mut self) -> () {
        match self.modual.current_env_id {
            Some(id) => self.modual.current_body_id = self.modual.environments[id].create_env(self.modual.current_body_id),
            None => {
                let new_id: usize = self.modual.mod_envs.len();
                let current_id: usize = self.modual.current_body_id;
                self.modual.mod_envs.push(Environment::new(new_id, Some(current_id)));
                self.modual.current_body_id = new_id;
                self.modual.mod_body_id = new_id;
            },
        };
    }

    fn check_if_all_bodies_return(&mut self) -> () {
        match self.modual.current_env_id {
            Some(id) => {
                if !self.modual.environments[id].check_if_all_bodies_return() {
                    let function: Function = self.get_function();
                    self.create_type_error(ErrorLevel::Error, 
                                           format!("Function {} dosen't return value in every branch",
                                                   function.identifier.get_fragment()),
                                           function.original,
                                           function.identifier.get_line(),
                                           function.identifier.get_offset());
                }
            },
            None => panic!("Fatal error in type checker!!!"),
        }; 
    }
}

