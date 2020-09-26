#![allow(dead_code)]

mod interpreter_env;
mod interpreter_func_env;
mod literal_interp;
mod operation_interp;
mod expression_interp;
mod statement_interp;

pub use super::span::Span;

pub use super::error::{
//    ErrorLevel,
//    Error,
    ErrorHandler,
};

pub use super::parser::{
    statement,
    Statement,
    expressions,
    operations,
    Literal,
    TypeDecleration,
};


pub use super::type_checker::{
    Modual,
};

pub use interpreter_env::InterpEnv;
pub use interpreter_func_env::InterpFuncEnv;

#[derive(Debug, Clone, PartialEq)]
pub struct Interpreter {
    pub error_handler: ErrorHandler,
    
    pub modual: Modual,

    pub env: InterpEnv,
    pub func_envs: Vec<InterpFuncEnv>,
}


impl Interpreter {
    pub fn interpret(modual: Modual) -> String {
        let mut interpreter: Interpreter = Interpreter{
            error_handler: ErrorHandler::new(true),
            
            modual: modual,
 
            env: InterpEnv::new(),
            func_envs: vec!(),
        };

        return "".to_string();//interpreter.interpret_statement(ast).to_string();
    }

    fn create_func_env(&mut self) {
        //TODO
    } 
    
    fn drop_func_env(&mut self) {
        //TODO
    } 
    
    fn get_variable(&mut self, name: String) -> Literal {    
        let result: Result<Literal, String> = match self.func_envs.len() {
            0 => Err("error".to_string()), 
            n => self.func_envs[n-1].get_variable(name.clone()),
        };
        match result {
            Ok(val) => return val,
            Err(_) => {
                match self.env.get_variable(name) {
                    Ok(val) => return val,
                    Err(msg) => panic!(msg), 
                };
            },

        };

    }

    fn store_variable(&mut self, name: Span<String>, value: Literal) -> () { 
        let result: bool = match self.func_envs.len() {
            0 => self.env.store_variable(name, value),
            n => self.func_envs[n-1].store_variable(name, value),
        };

        if !result {
            //TODO Create error,
        }
    }
    
    fn assign_variable(&mut self, name: Span<String>, value: Literal) -> () {
        let result: bool = match self.func_envs.len() {
            0 => self.env.assign_variable(name, value),
            n => self.func_envs[n-1].assign_variable(name, value),
        };

        if !result {
            //TODO Create error,
        }
    }
}

