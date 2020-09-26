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
    statement::Function,
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
    pub fn interpret(modual: Modual) -> () {
        let mut interpreter: Interpreter = Interpreter{
            error_handler: ErrorHandler::new(true),
            
            modual: modual,
 
            env: InterpEnv::new(),
            func_envs: vec!(),
        };
        let ast: Statement = interpreter.modual.ast.clone();
        interpreter.interpret_statement(ast);
    }

    fn create_func_env(&mut self, func_id: usize) -> () {
        self.func_envs.push(InterpFuncEnv::new(func_id));
        self.create_env();
    } 
    
    fn drop_func_env(&mut self) -> () {
        self.func_envs.pop();
    } 
    
    fn create_env(&mut self) -> () {
        match self.func_envs.len() {
            0 => panic!("fatal interpreter error"), 
            n => self.func_envs[n-1].create_env(),
        };
    } 
    
    fn drop_env(&mut self) -> () {
        match self.func_envs.len() {
            0 => panic!("fatal interpreter error"), 
            n => self.func_envs[n-1].drop_env(),
        };
    } 

    fn get_function(&mut self, name: String, body_id: usize) -> Function {
        let mut func_id: usize;
        match self.func_envs.len() {
            0 => {
                match self.modual.environment.lookup_function(name) {
                    Ok(id) => return self.modual.environments[id].function.clone(),
                    Err(msg) => panic!(msg),
                };
            },
            n => func_id = self.func_envs[n-1].func_id,
        };
        let result: Result<usize, String> = self.modual.environments[func_id].lookup_function_id(name.clone(), body_id); 
        match result {
            Ok(id) => return self.modual.environments[id].function.clone(),
            Err(_) => {
                match self.modual.environment.lookup_function(name) {
                    Ok(id) => return self.modual.environments[id].function.clone(),
                    Err(msg) => panic!(msg),
                };
            },
        };
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
            panic!("Fatal interpreter error")
            //TODO Create error,
        }
    }
    
    fn assign_variable(&mut self, name: Span<String>, value: Literal) -> () {
        let result: bool = match self.func_envs.len() {
            0 => false,
            n => self.func_envs[n-1].assign_variable(name.clone(), value.clone()),
        };

        if !result {
            if self.env.assign_variable(name, value) {
                panic!("Fatal interpreter error")
                //TODO Create error,
            }
        }
    }
}

