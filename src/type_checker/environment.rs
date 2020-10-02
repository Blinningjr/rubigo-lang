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
    pub variables: Vec<Variable>, 

    pub returns_value: bool,
    pub if_body: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub mutable: bool,
    pub identifier: Span<String>,
    pub r#type: Type,
}

impl Variable {
    pub fn new(identifier: Span<String>, r#type: Type, mutable: bool) -> Variable {
        return Variable {
            mutable: mutable,
            identifier: identifier,
            r#type: r#type,
        };
    }
}


impl Environment {
    pub fn new(id: usize, previus_id: Option<usize>) -> Environment {
        return Environment{
            id: id,
            previus_id: previus_id,
            functions: vec!(),
            variables: vec!(),

            returns_value: false,
            if_body: false,
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

    pub fn add_variable(&mut self, variable: Variable) -> bool {
        self.variables.push(variable);
        return true;
    }

    pub fn lookup_variable(&mut self, identifier: String) -> Result<Variable, String> {
        for var in self.variables.iter() {
            if var.identifier.get_fragment() == identifier {
                return Ok(var.clone());
            }
        }
        return Err(format!("Variable {:#?} not in scope.", identifier));
    }
}

