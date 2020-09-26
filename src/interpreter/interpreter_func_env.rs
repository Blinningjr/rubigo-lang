#![allow(dead_code)]

pub use super::{
    Span,
    InterpEnv,
    Literal,
};


#[derive(Debug, Clone, PartialEq)]
pub struct InterpFuncEnv {
    pub envs: Vec<InterpEnv>, 
    pub func_id: usize,

}

impl InterpFuncEnv {
    pub fn new(func_id: usize) -> InterpFuncEnv {
        return InterpFuncEnv {
            envs: vec!(),
            func_id: func_id,

        };
    }
    
    pub fn get_variable(&mut self, name: String) -> Result<Literal, String> {
        for i in self.envs.len()..0 {
            let result: Result<Literal, String> = self.envs[i].get_variable(name.clone());
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
        match self.envs.len() {
            0 => panic!("Fatal Interpreter error"),
            n => return self.envs[n-1].assign_variable(name, value),
        };
    }
}

