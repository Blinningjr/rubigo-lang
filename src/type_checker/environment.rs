#![allow(dead_code)]

pub use super::{
    Span,
    r#type,
};

pub use r#type::{
    Type,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub id: usize,
    pub previus_id: Option<usize>,
    pub functions: Vec<(Span<String>, usize)>, 
    pub variables: Vec<(Span<String>, Type)>, 
}


impl Environment {
    pub fn new(id: usize, previus_id: Option<usize>) -> Environment {
        return Environment{
            id: id,
            previus_id: previus_id,
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
        self.variables.push((identifier, Type::Custom(r#type.get_fragment())));
        return true;
    }

    pub fn lookup_variable(&mut self, identifier: String) -> Result<Type, String> {
        for (ident, r#type) in self.variables.iter() {
            if ident.get_fragment() == identifier {
                return Ok(r#type.clone());
            }
        }
        return Err(format!("Variable {:#?} not in scope.", identifier));
    }
}

