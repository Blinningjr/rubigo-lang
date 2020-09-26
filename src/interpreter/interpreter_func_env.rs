#![allow(dead_code)]

pub use super::{
    Span,
    InterpEnv,
    Literal,
};


#[derive(Debug, Clone, PartialEq)]
pub struct InterpFuncEnv {
    pub func_id: usize,
    pub envs: Vec<InterpEnv>, 
}

impl InterpFuncEnv {
    pub fn new(func_id: usize) -> InterpFuncEnv {
        return InterpFuncEnv {
            func_id: func_id,
            envs: vec!(),
        };
    }
    
    pub fn create_env(&mut self) -> () {
        self.envs.push(InterpEnv::new());
    } 
    
    pub fn drop_env(&mut self) -> () {
        self.envs.pop();
    } 
    
    pub fn get_variable(&mut self, name: String) -> Result<Literal, String> {
        for env in self.envs.iter().rev() {
            let result: Result<Literal, String> = env.get_variable(name.clone());
            match result {
                Ok(val) => return Ok(val),
                Err(_) => (),
            };
        }
        return Err(format!("Error: Variable {} not found", name));
    }

    pub fn store_variable(&mut self, name: Span<String>, value: Literal) -> bool {
        match self.envs.len() {
            0 => panic!("Fatal Interpreter error"),
            n => return self.envs[n-1].store_variable(name, value),
        };
    }
    
    pub fn assign_variable(&mut self, name: Span<String>, value: Literal) -> bool {
        for i in self.envs.len()..0 {
            let result: bool = self.envs[i].assign_variable(name.clone(), value.clone());
            if result {
                return result
            }
        }
        return false;
    }
}

