#![allow(dead_code)]

pub use super::{
    Span,
    Literal,
};


#[derive(Debug, Clone, PartialEq)]
pub struct InterpEnv {
    pub variables: Vec<(Span<String>, Literal)>, 
}


impl InterpEnv {
    pub fn new() -> InterpEnv {
        return InterpEnv{
            variables: vec!(),
        };
    }

    pub fn get_variable(&mut self, name: String) -> Result<Literal, String> {
        for var in self.variables.iter() {
            if var.0.get_fragment() == name {
                return Ok(var.1.clone());
            }
        }
        return Err(format!("Error: Variable {} not found", name));
    }

    pub fn store_variable(&mut self, name: Span<String>, value: Literal) -> bool {
        match self.get_variable(name.get_fragment()) {
            Ok(_) => return false,
            Err(_) => {
                self.variables.push((name, value));
                return true;
            },
        };
    }

    pub fn assign_variable(&mut self, name: Span<String>, value: Literal) -> bool {
        for i in 0..self.variables.len() {
            if self.variables[i].0.get_fragment() == name.get_fragment() {
                self.variables[i] = (name, value); 
                return true;
            }
        }
        return false;
    }
}
