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
    ModualBody,
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

    pub envs: Vec<InterpEnv>,
    pub func_envs: Vec<InterpFuncEnv>,
}


impl Interpreter {
    pub fn interpret(modual: Modual) -> () {
        let mut interpreter: Interpreter = Interpreter{
            error_handler: ErrorHandler::new(true),
            
            modual: modual,
 
            envs: vec!(InterpEnv::new()),
            func_envs: vec!(),
        };
        let ast: ModualBody = interpreter.modual.ast.clone();
        interpreter.interpret_modual_body(ast);
    }

    fn interpret_modual_body(&mut self, mod_body: ModualBody) {
        for stmt in mod_body.body.iter() {
            self.interpret_statement(stmt.clone());
        }
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
        let func_id: usize;
        match self.func_envs.len() {
            0 => {
                let mut body_id_r: Option<usize> = Some(body_id);
                loop {
                    match body_id_r {
                        Some(b_id) => {
                            match self.modual.mod_envs[b_id].lookup_function(name.clone()) {
                                Ok(id) => return self.modual.environments[id].function.clone(),
                                Err(_) => body_id_r = self.modual.mod_envs[b_id].previus_id
                            };
                        },
                        None => panic!("fatal interpreter error"),
                    };
                }
            },
            n => func_id = self.func_envs[n-1].func_id,
        };
        let result: Result<usize, String> = self.modual.environments[func_id].lookup_function_id(name.clone(), body_id); 
        match result {
            Ok(id) => return self.modual.environments[id].function.clone(),
            Err(_) => {
                let mut body_id_r: Option<usize> = Some(body_id);
                loop {
                    match body_id_r {
                        Some(b_id) => {
                            match self.modual.mod_envs[b_id].lookup_function(name.clone()) {
                                Ok(id) => return self.modual.environments[id].function.clone(),
                                Err(_) => body_id_r = self.modual.mod_envs[b_id].previus_id
                            };
                        },
                        None => panic!("fatal interpreter error"),
                    };
                }
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
                for i in (0..self.envs.len()).rev() {
                    match self.envs[i].get_variable(name.clone()) {
                        Ok(val) => return val,
                        Err(_) => (), 
                    };
                } 
                panic!("fatal interpreter error");
            },

        };

    }

    fn store_variable(&mut self, name: Span<String>, value: Literal) -> () { 
        let result: bool = match self.func_envs.len() {
            0 => {
                let length: usize = self.envs.len();
                self.envs[length - 1].store_variable(name, value)
            },
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
            for i in (0..self.envs.len()).rev() {
                if self.envs[i].assign_variable(name.clone(), value.clone()) {
                    return 
                }
            } 
            panic!("fatal interpreter error");
        }
    }
}

