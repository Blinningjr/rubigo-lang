#![allow(dead_code)]

pub use super::{
    Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub environment_id: usize,
    pub previus_env_id: Option<usize>,
    pub functions: Vec<(Span<String>, usize)>, 
    pub variables: Vec<(Span<String>, String)>, 
}


impl Environment {
    pub fn new(environment_id: usize, previus_env_id: Option<usize>) -> Environment {
        return Environment{
            environment_id: environment_id,
            previus_env_id: previus_env_id,
            functions: vec!(),
            variables: vec!(),
        };
    }

    pub fn add_function(&mut self, identifier: Span<String>, env_id: usize) -> bool {
        self.functions.push((identifier, env_id));
        return true;
    }

    pub fn lookup_function(&mut self, identifier: String) -> Result<usize, String> {
        for (ident, function_env_id) in self.functions.iter() {
            if ident.get_fragment() == identifier {
                return Ok(*function_env_id);
            }
        }
        return Err(format!("Function {:#?} not in scope.", identifier));
    }

    pub fn add_variable(&mut self, identifier: Span<String>, r#type: Span<String>) -> bool {
        self.variables.push((identifier, r#type.get_fragment()));
        return true;
    }

    pub fn lookup_variable(&mut self, identifier: String) -> Result<String, String> {
        for (ident, r#type) in self.variables.iter() {
            if ident.get_fragment() == identifier {
                return Ok(r#type.clone());
            }
        }
        return Err(format!("Variable {:#?} not in scope.", identifier));
    }
}

